use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, specta::Type)]
pub enum Theme {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "system")]
    System,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, specta::Type)]
pub enum UpdateChannel {
    Stable,
    PreRelease,
}

impl UpdateChannel {
    pub fn as_str(&self) -> &str {
        match self {
            UpdateChannel::Stable => "stable",
            UpdateChannel::PreRelease => "pre-release",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum LanguageCode {
    #[serde(rename = "ja-JP", alias = "jaJp")] // alias is for legacy support
    JaJp,
    #[serde(rename = "en-US", alias = "enUs")] // alias is for legacy support
    EnUs,
    #[serde(rename = "en-GB", alias = "enGb")] // alias is for legacy support
    EnGb,
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "user-provided")]
    UserProvided(String),
}

impl LanguageCode {
    pub fn code(&self) -> &str {
        match self {
            LanguageCode::JaJp => "ja-JP",
            LanguageCode::EnUs => "en-US",
            LanguageCode::EnGb => "en-GB",
            LanguageCode::ZhCn => "zh-CN",
            LanguageCode::UserProvided(code) => code,
        }
    }

    pub fn booth_lang_code(&self) -> &str {
        match self {
            LanguageCode::JaJp => "ja",
            LanguageCode::EnUs | LanguageCode::EnGb => "en",
            LanguageCode::ZhCn => "zh-cn",
            LanguageCode::UserProvided(_) => "en",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PreferenceStore {
    #[serde(skip)]
    pub file_path: PathBuf,

    pub data_dir_path: PathBuf,
    pub theme: Theme,
    pub language: LanguageCode,

    pub delete_on_import: bool,
    pub zip_extraction: bool,
    pub use_unitypackage_selected_open: bool,
    pub use_trash_bin: bool,

    pub update_channel: UpdateChannel,

    // クラウド同期環境でこのインストールを識別するための情報。
    // device_id は初回起動時に生成されたら変わらない値で、per-device のメタデータファイル名や
    // アセットの registered_device_id に使われる。device_name はユーザーに見せる表示名。
    pub device_id: Uuid,
    pub device_name: String,

    // 0 の場合は自動更新を無効化する
    pub auto_refresh_interval_seconds: u32,
}

pub fn default_device_name() -> String {
    hostname::get()
        .ok()
        .and_then(|name| name.into_string().ok())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "Unknown Device".to_string())
}

impl PreferenceStore {
    pub fn default<P, Q>(preference_file_path: P, default_data_dir_path: Q) -> Self
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
        let locale = match locale.as_str() {
            "ja-JP" => LanguageCode::JaJp,
            "en-US" => LanguageCode::EnUs,
            "en-GB" => LanguageCode::EnGb,
            "zh-CN" => LanguageCode::ZhCn,
            _ => LanguageCode::EnUs,
        };

        Self {
            file_path: preference_file_path.as_ref().to_path_buf(),

            data_dir_path: default_data_dir_path.as_ref().to_path_buf(),
            theme: Theme::System,
            language: locale,

            delete_on_import: false,
            zip_extraction: true,
            use_unitypackage_selected_open: true,
            use_trash_bin: true,

            update_channel: UpdateChannel::Stable,

            device_id: Uuid::new_v4(),
            device_name: default_device_name(),

            auto_refresh_interval_seconds: 0,
        }
    }

    pub fn get_data_dir(&self) -> &PathBuf {
        &self.data_dir_path
    }

    pub fn set_data_dir(&mut self, data_dir_path: PathBuf) {
        self.data_dir_path = data_dir_path;
    }

    pub fn overwrite(&mut self, other: &Self) {
        self.data_dir_path = other.data_dir_path.clone();
        self.theme = other.theme;
        self.delete_on_import = other.delete_on_import;
        self.zip_extraction = other.zip_extraction;
        self.use_unitypackage_selected_open = other.use_unitypackage_selected_open;
        self.use_trash_bin = other.use_trash_bin;
        self.update_channel = other.update_channel;
        self.auto_refresh_interval_seconds = other.auto_refresh_interval_seconds;

        // device_id / device_name は端末固有の情報なので other 側の値では上書きしない。

        // If the new language is user-provided, skip updating the language field to prevent corruption.
        if let LanguageCode::UserProvided(_) = other.language {
            // Skip updating the language field.
        } else {
            self.language = other.language.clone();
        }
    }
}
