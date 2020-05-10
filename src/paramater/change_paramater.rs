use crate::{flag::Condition, types::ChangeKey};
use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChangeParamater<T>
where
    T: AnimationFile,
{
    condition: Vec<Condition>,
    #[serde(bound(
        deserialize = "T::PackKey: Deserialize<'de>, T::AnimationKey: Deserialize<'de>",
        serialize = "T::PackKey: Serialize, T::AnimationKey: Serialize",
    ))]
    change_key: ChangeKey<T>,
}

impl<T> ChangeParamater<T>
where
    T: AnimationFile,
{
    pub fn valid_change_key<P>(
        &self,
        current: T::PackKey,
        predicate: P,
    ) -> Option<(T::PackKey, T::AnimationKey)>
    where
        P: FnMut(Condition) -> bool + Copy,
    {
        if self.valid_condition(predicate) == true {
            Some(self.change_key(current))
        } else {
            None
        }
    }

    fn change_key(&self, current: T::PackKey) -> (T::PackKey, T::AnimationKey) {
        (
            self.change_key.pack.unwrap_or(current),
            self.change_key.animation,
        )
    }

    fn valid_condition<P>(&self, mut predicate: P) -> bool
    where
        P: FnMut(Condition) -> bool + Copy,
    {
        // 条件がなければ無条件OK

        if self.condition.len() == 0 {
            return true;
        }

        self.condition.iter().fold(false, move |valid, &condition| {
            valid || predicate(condition)
        })
    }
}
