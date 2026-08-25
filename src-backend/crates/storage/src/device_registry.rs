use std::{collections::HashMap, path::Path};

use uuid::Uuid;

const PREFIX: &str = "device";

fn device_filename(device_id: Uuid) -> String {
    format!("{}__{}.json", PREFIX, device_id)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DeviceEntry {
    id: Uuid,
    name: String,
}

// このデバイスの表示名を、クラウド同期対象の metadata ディレクトリに書き出す。
// 他デバイスがフィルタの選択肢としてこのデバイス名を参照できるようにするための、
// アセットとは独立した小さな per-device ファイル。
pub async fn save_device_name<P: AsRef<Path>>(
    data_dir: P,
    device_id: Uuid,
    device_name: &str,
) -> Result<(), String> {
    let metadata_dir = data_dir.as_ref().join("metadata");

    if !metadata_dir.exists() {
        std::fs::create_dir_all(&metadata_dir)
            .map_err(|e| format!("Failed to create metadata dir: {}", e))?;
    }

    let path = metadata_dir.join(device_filename(device_id));

    let entry = DeviceEntry {
        id: device_id,
        name: device_name.to_string(),
    };

    let bytes =
        serde_json::to_vec(&entry).map_err(|e| format!("Failed to serialize device entry: {}", e))?;

    file::modify_guard::write_atomic(&path, &bytes)
        .await
        .map_err(|e| format!("Failed to write device entry: {}", e))
}

// metadata ディレクトリ内の全 per-device ファイルを読み、device_id -> device_name の
// 対応表を組み立てる。
pub async fn load_device_names<P: AsRef<Path>>(data_dir: P) -> Result<HashMap<Uuid, String>, String> {
    let metadata_dir = data_dir.as_ref().join("metadata");

    if !metadata_dir.exists() {
        return Ok(HashMap::new());
    }

    let prefix = format!("{}__", PREFIX);

    let mut result = HashMap::new();

    let entries = std::fs::read_dir(&metadata_dir)
        .map_err(|e| format!("Failed to read metadata dir: {}", e))?;

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();

        let is_device_file = path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with(&prefix) && name.ends_with(".json"))
            .unwrap_or(false);

        if !is_device_file {
            continue;
        }

        let file = match std::fs::File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                log::warn!("Failed to open device entry file {}: {}", path.display(), e);
                continue;
            }
        };

        let entry: DeviceEntry = match serde_json::from_reader(file) {
            Ok(entry) => entry,
            Err(e) => {
                log::warn!(
                    "Failed to deserialize device entry file {}: {}",
                    path.display(),
                    e
                );
                continue;
            }
        };

        result.insert(entry.id, entry.name);
    }

    Ok(result)
}
