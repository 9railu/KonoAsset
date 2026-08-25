use std::{collections::HashMap, sync::Arc};

use storage::asset_storage::AssetStorage;
use tauri::{State, async_runtime::Mutex};
use uuid::Uuid;

#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum CloudAvailabilityStatus {
    // クラウド同期クライアントによりローカルにダウンロードされていない
    NotDownloaded,
    // 一部のファイルのみローカルに存在する
    PartiallyAvailable,
    // ローカルに完全に存在する（同期非対象のフォルダ、または Windows 以外の場合も含む）
    FullyAvailable,
}

// アセット本体（data/{id}）が Google Drive / OneDrive 等のクラウド同期クライアントによって
// ローカルにダウンロード済みかどうかを判定する。正確な進捗率（%）は OS から取得できないため、
// 3段階の大まかな状態のみを返す。Windows 以外のビルドでは常に FullyAvailable を返す。
#[tauri::command]
#[specta::specta]
pub async fn get_asset_availability_statuses(
    basic_store: State<'_, Arc<Mutex<AssetStorage>>>,
    ids: Vec<Uuid>,
) -> Result<HashMap<Uuid, CloudAvailabilityStatus>, String> {
    let data_dir = basic_store.lock().await.data_dir();

    let mut result = HashMap::new();

    for id in ids {
        let asset_dir = data_dir.join("data").join(id.to_string());
        result.insert(id, cloud_status::check_dir_availability(&asset_dir));
    }

    Ok(result)
}

#[cfg(windows)]
mod cloud_status {
    use std::{os::windows::ffi::OsStrExt, path::Path};

    use windows::{
        Win32::Storage::FileSystem::{
            FILE_ATTRIBUTE_OFFLINE, FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS,
            FILE_ATTRIBUTE_RECALL_ON_OPEN, GetFileAttributesW, INVALID_FILE_ATTRIBUTES,
        },
        core::PCWSTR,
    };

    use super::CloudAvailabilityStatus;

    // 一つのフォルダ内でチェックするファイル数の上限。アセットフォルダには
    // 数百〜数千ファイル含まれることがあるため、全件走査せずおおまかな判定に留める。
    const SAMPLE_LIMIT: usize = 50;

    fn is_placeholder(attrs: u32) -> bool {
        (attrs & FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS.0) != 0
            || (attrs & FILE_ATTRIBUTE_OFFLINE.0) != 0
            || (attrs & FILE_ATTRIBUTE_RECALL_ON_OPEN.0) != 0
    }

    fn get_attributes(path: &Path) -> Option<u32> {
        let wide: Vec<u16> = path
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let attrs = unsafe { GetFileAttributesW(PCWSTR(wide.as_ptr())) };

        if attrs == INVALID_FILE_ATTRIBUTES {
            None
        } else {
            Some(attrs)
        }
    }

    pub fn check_dir_availability(dir: &Path) -> CloudAvailabilityStatus {
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(_) => return CloudAvailabilityStatus::FullyAvailable,
        };

        let mut any_placeholder = false;
        let mut any_available = false;
        let mut checked = 0;

        for entry in entries.filter_map(|e| e.ok()) {
            if checked >= SAMPLE_LIMIT {
                break;
            }
            checked += 1;

            match get_attributes(&entry.path()) {
                Some(attrs) if is_placeholder(attrs) => any_placeholder = true,
                _ => any_available = true,
            }
        }

        if checked == 0 {
            CloudAvailabilityStatus::FullyAvailable
        } else if any_placeholder && any_available {
            CloudAvailabilityStatus::PartiallyAvailable
        } else if any_placeholder {
            CloudAvailabilityStatus::NotDownloaded
        } else {
            CloudAvailabilityStatus::FullyAvailable
        }
    }
}

#[cfg(not(windows))]
mod cloud_status {
    use std::path::Path;

    use super::CloudAvailabilityStatus;

    pub fn check_dir_availability(_dir: &Path) -> CloudAvailabilityStatus {
        CloudAvailabilityStatus::FullyAvailable
    }
}
