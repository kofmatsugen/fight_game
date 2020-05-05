#[cfg(feature = "serialize")]
use failure::Fail;
use serde::{Deserialize, Serialize};
#[cfg(feature = "serialize")]
use std::str::FromStr;

#[cfg(feature = "serialize")]
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "unknown pack name: {}", _0)]
    UnknownPackName(String),
    #[fail(display = "unknown animation name: {}", _0)]
    UnknownAnimationName(String),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PackKey {
    Base,
    TestPack,
}

#[cfg(feature = "serialize")]
impl FromStr for PackKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Basic" => Ok(PackKey::Base),
            "test_pack" => Ok(PackKey::TestPack),
            _ => Err(Error::UnknownPackName(s.into())),
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnimationKey {
    //
    BasePose,
    Punch,
    Stance,
    StartUp,
    Walk,
    Back,
    //
    Deform,
}

#[cfg(feature = "serialize")]
impl FromStr for AnimationKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BasePose" => Ok(AnimationKey::BasePose),
            "Punch" => Ok(AnimationKey::Punch),
            "Stance" => Ok(AnimationKey::Stance),
            "StartUp" => Ok(AnimationKey::StartUp),
            "Walk" => Ok(AnimationKey::Walk),
            "Back" => Ok(AnimationKey::Back),

            "deform" => Ok(AnimationKey::Deform),
            _ => Err(Error::UnknownAnimationName(s.into())),
        }
    }
}
