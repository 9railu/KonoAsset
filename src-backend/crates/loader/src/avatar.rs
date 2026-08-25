use model::Avatar;
use monostate::MustBe;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use super::share::{LegacyAssetDescriptionV1, LegacyAssetDescriptionV2, LegacyAssetDescriptionV3};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VersionedAvatars {
    Avatars {
        version: MustBe!(4u64),
        data: HashSet<Avatar>,
    },
    LegacyAvatarsV3 {
        version: MustBe!(3u64),
        data: HashSet<LegacyAvatarV3>,
    },
    LegacyAvatarsV2 {
        version: MustBe!(2u64),
        data: HashSet<LegacyAvatarV2>,
    },
    LegacyAvatarsV1(HashSet<LegacyAvatarV1>),
}

impl TryInto<HashSet<Avatar>> for VersionedAvatars {
    type Error = String;

    fn try_into(self) -> Result<HashSet<Avatar>, Self::Error> {
        match self {
            VersionedAvatars::Avatars { data, .. } => Ok(data),
            VersionedAvatars::LegacyAvatarsV3 { data, .. } => {
                Ok(data.into_iter().map(|legacy| legacy.into()).collect())
            }
            VersionedAvatars::LegacyAvatarsV2 { data, .. } => {
                let mut avatars = HashSet::new();
                for legacy_avatar in data {
                    let v3_avatar: LegacyAvatarV3 = legacy_avatar.try_into()?;
                    avatars.insert(v3_avatar.into());
                }
                Ok(avatars)
            }
            VersionedAvatars::LegacyAvatarsV1(legacy_avatars) => {
                let mut avatars = HashSet::new();
                for legacy_avatar in legacy_avatars {
                    let v2_avatar: LegacyAvatarV2 = legacy_avatar.try_into()?;
                    let v3_avatar: LegacyAvatarV3 = v2_avatar.try_into()?;

                    avatars.insert(v3_avatar.into());
                }
                Ok(avatars)
            }
        }
    }
}

impl TryFrom<HashSet<Avatar>> for VersionedAvatars {
    type Error = String;

    fn try_from(value: HashSet<Avatar>) -> Result<VersionedAvatars, Self::Error> {
        Ok(VersionedAvatars::Avatars {
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
pub struct LegacyAvatarV3 {
    pub id: Uuid,
    pub description: LegacyAssetDescriptionV3,
}

impl Into<Avatar> for LegacyAvatarV3 {
    fn into(self) -> Avatar {
        Avatar {
            id: self.id,
            description: self.description.into(),
        }
    }
}

/*
 * V2
 */
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LegacyAvatarV2 {
    pub id: Uuid,
    pub description: LegacyAssetDescriptionV2,
}

impl TryInto<LegacyAvatarV3> for LegacyAvatarV2 {
    type Error = String;

    fn try_into(self) -> Result<LegacyAvatarV3, Self::Error> {
        Ok(LegacyAvatarV3 {
            id: self.id,
            description: self.description.try_into()?,
        })
    }
}

/*
 * V1
 */
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LegacyAvatarV1 {
    pub id: Uuid,
    pub description: LegacyAssetDescriptionV1,
}

impl TryInto<LegacyAvatarV2> for LegacyAvatarV1 {
    type Error = String;

    fn try_into(self) -> Result<LegacyAvatarV2, Self::Error> {
        Ok(LegacyAvatarV2 {
            id: self.id,
            description: self.description.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_avatar_v1_to_v2_migration() {
        let image_path = if cfg!(target_os = "windows") {
            "C:\\path\\to\\image.png".to_string()
        } else {
            "/path/to/image.png".to_string()
        };

        let legacy = LegacyAvatarV1 {
            id: Uuid::new_v4(),
            description: LegacyAssetDescriptionV1 {
                name: "name".into(),
                creator: "creator".into(),
                image_path: Some(image_path),
                image_filename: None,
                booth_item_id: Some(123),
                tags: vec!["tag".into()],
                created_at: 12345,
                published_at: Some(67890),
            },
        };

        let latest: LegacyAvatarV2 = legacy.try_into().unwrap();

        assert_eq!(
            latest.description.image_filename,
            Some("image.png".to_string()),
        );
    }
}
