use model::OtherAsset;
use monostate::MustBe;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use super::share::LegacyAssetDescriptionV3;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VersionedOtherAssets {
    OtherAssets {
        version: MustBe!(4u64),
        data: HashSet<OtherAsset>,
    },
    LegacyOtherAssetsV3 {
        version: MustBe!(3u64),
        data: HashSet<LegacyOtherAssetV3>,
    },
}

impl TryInto<HashSet<OtherAsset>> for VersionedOtherAssets {
    type Error = String;

    fn try_into(self) -> Result<HashSet<OtherAsset>, Self::Error> {
        match self {
            VersionedOtherAssets::OtherAssets { data, .. } => Ok(data),
            VersionedOtherAssets::LegacyOtherAssetsV3 { data, .. } => {
                Ok(data.into_iter().map(|legacy| legacy.into()).collect())
            }
        }
    }
}

impl TryFrom<HashSet<OtherAsset>> for VersionedOtherAssets {
    type Error = String;

    fn try_from(value: HashSet<OtherAsset>) -> Result<VersionedOtherAssets, Self::Error> {
        Ok(VersionedOtherAssets::OtherAssets {
            version: MustBe!(4u64),
            data: value,
        })
    }
}

/*
 * V3
 */

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LegacyOtherAssetV3 {
    pub id: Uuid,
    pub description: LegacyAssetDescriptionV3,
    pub category: String,
}

impl Into<OtherAsset> for LegacyOtherAssetV3 {
    fn into(self) -> OtherAsset {
        OtherAsset {
            id: self.id,
            description: self.description.into(),
            category: self.category,
        }
    }
}
