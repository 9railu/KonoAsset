use uuid::Uuid;

use crate::base::{AssetDescription, AssetType, Avatar, AvatarWearable, OtherAsset, WorldObject};

pub trait AssetTrait {
    // 旧バージョンで使われていた固定ファイル名。新規デバイスのファイル書き込みには使わず、
    // 移行処理（旧ファイルを最初に起動したデバイスの per-device ファイルへ引き継ぐ）専用。
    fn filename() -> String;

    // per-device ファイルの共通プレフィックス（例: "avatars"）。
    fn filename_prefix() -> String;

    // 指定したデバイス用の per-device ファイル名を組み立てる。
    fn device_filename(device_id: Uuid) -> String {
        format!("{}__{}.json", Self::filename_prefix(), device_id)
    }

    fn asset_type() -> AssetType;

    fn get_id(&self) -> Uuid;
    fn set_id(&mut self, id: Uuid);
    fn get_description(&self) -> &AssetDescription;
    fn get_description_as_mut(&mut self) -> &mut AssetDescription;
}

impl AssetTrait for Avatar {
    fn filename() -> String {
        "avatars.json".into()
    }

    fn filename_prefix() -> String {
        "avatars".into()
    }

    fn asset_type() -> AssetType {
        AssetType::Avatar
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    fn get_description(&self) -> &AssetDescription {
        &self.description
    }

    fn get_description_as_mut(&mut self) -> &mut AssetDescription {
        &mut self.description
    }
}

impl AssetTrait for AvatarWearable {
    fn filename() -> String {
        "avatarWearables.json".into()
    }

    fn filename_prefix() -> String {
        "avatarWearables".into()
    }

    fn asset_type() -> AssetType {
        AssetType::AvatarWearable
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    fn get_description(&self) -> &AssetDescription {
        &self.description
    }

    fn get_description_as_mut(&mut self) -> &mut AssetDescription {
        &mut self.description
    }
}

impl AssetTrait for WorldObject {
    fn filename() -> String {
        "worldObjects.json".into()
    }

    fn filename_prefix() -> String {
        "worldObjects".into()
    }

    fn asset_type() -> AssetType {
        AssetType::WorldObject
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    fn get_description(&self) -> &AssetDescription {
        &self.description
    }

    fn get_description_as_mut(&mut self) -> &mut AssetDescription {
        &mut self.description
    }
}

impl AssetTrait for OtherAsset {
    fn filename() -> String {
        "otherAssets.json".into()
    }

    fn filename_prefix() -> String {
        "otherAssets".into()
    }

    fn asset_type() -> AssetType {
        AssetType::OtherAsset
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    fn get_description(&self) -> &AssetDescription {
        &self.description
    }

    fn get_description_as_mut(&mut self) -> &mut AssetDescription {
        &mut self.description
    }
}
