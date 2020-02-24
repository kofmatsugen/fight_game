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
}

#[cfg(feature = "serialize")]
impl FromStr for PackKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sample" => Ok(PackKey::Base),
            _ => Err(Error::UnknownPackName(s.into())),
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnimationKey {
    Stance,
    Sit,
    Walk,
    Run,
    Defence,
    Dead2,
    Dead1,
    Kick1,
    Kick2,
    Punch1,
    Punch2,
    Sitdown,
    Standup,
}

#[cfg(feature = "serialize")]
impl FromStr for AnimationKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0000_stance" => Ok(AnimationKey::Stance),
            "0001_sit" => Ok(AnimationKey::Sit),
            "0002_walk" => Ok(AnimationKey::Walk),
            "0003_run" => Ok(AnimationKey::Run),
            "0004_defense" => Ok(AnimationKey::Defence),
            "0005_dead2" => Ok(AnimationKey::Dead1),
            "0006_dead1" => Ok(AnimationKey::Dead2),
            "0007_kick1" => Ok(AnimationKey::Kick1),
            "0008_kick2" => Ok(AnimationKey::Kick2),
            "0009_punch1" => Ok(AnimationKey::Punch1),
            "0010_punch2" => Ok(AnimationKey::Punch2),
            "0011_sitdown" => Ok(AnimationKey::Sitdown),
            "0012_standup" => Ok(AnimationKey::Standup),
            _ => Err(Error::UnknownAnimationName(s.into())),
        }
    }
}