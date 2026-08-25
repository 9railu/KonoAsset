use std::sync::Arc;

use model::preference::PreferenceStore;
use storage::{asset_storage::AssetStorage, definitions::FilterRequest, device_registry, search};
use tauri::{State, async_runtime::Mutex};
use uuid::Uuid;

#[tauri::command]
#[specta::specta]
pub async fn get_filtered_asset_ids(
    basic_store: State<'_, Arc<Mutex<AssetStorage>>>,
    request: FilterRequest,
) -> Result<Vec<Uuid>, String> {
    let basic_store = basic_store.lock().await;
    Ok(search::filter(&basic_store, &request).await)
}

#[derive(serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub id: Uuid,
    pub name: String,
}

// クラウド同期フォルダ上にある全デバイスの名前一覧を返す（「PCで絞り込み」の選択肢用）。
// 呼び出しのついでに、このデバイス自身の現在の表示名も書き出しておく。
#[tauri::command]
#[specta::specta]
pub async fn get_registered_device_names(
    basic_store: State<'_, Arc<Mutex<AssetStorage>>>,
    preference: State<'_, Arc<Mutex<PreferenceStore>>>,
) -> Result<Vec<DeviceInfo>, String> {
    let data_dir = basic_store.lock().await.data_dir();

    let (device_id, device_name) = {
        let preference = preference.lock().await;
        (preference.device_id, preference.device_name.clone())
    };

    device_registry::save_device_name(&data_dir, device_id, &device_name).await?;

    let names = device_registry::load_device_names(&data_dir).await?;

    Ok(names
        .into_iter()
        .map(|(id, name)| DeviceInfo { id, name })
        .collect())
}
