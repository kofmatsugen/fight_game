use crate::components::{Damaged, Knockback};
use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};
use amethyst_sprite_studio::{components::AnimationTime, traits::animation_file::AnimationFile};
use std::marker::PhantomData;

// ノックバックの時間管理，ノックバック終了時にクリアする情報のクリアを行う
pub struct KnockbackSystem<T>
where
    T: AnimationFile,
{
    _translation: PhantomData<T>,
}

impl<T> KnockbackSystem<T>
where
    T: AnimationFile,
{
    pub fn new() -> Self {
        KnockbackSystem {
            _translation: PhantomData,
        }
    }
}

impl<'s, T> System<'s> for KnockbackSystem<T>
where
    T: AnimationFile,
{
    type SystemData = (
        Read<'s, Time>,
        Entities<'s>,
        ReadStorage<'s, AnimationTime>,
        WriteStorage<'s, Damaged<T>>,
        WriteStorage<'s, Knockback>,
    );

    fn run(
        &mut self,
        (_time, entities, animation_time, mut damaged, mut knockback): Self::SystemData,
    ) {
        #[cfg(not(feature = "count-frame"))]
        let time = _time.delta_seconds();
        #[cfg(feature = "count-frame")]
        let time = 1. / 60.;
        for (e, animation_time, knockback) in (&*entities, &animation_time, &mut knockback).join() {
            // ヒットストップがあるので，アニメーション再生中のみノックバックを計算
            if animation_time.is_play() == false {
                continue;
            }

            if knockback.is_knockback() == true {
                knockback.decrement(time);

                if knockback.is_knockback() == false {
                    // ノックバックしないようになったのでダメージ情報をクリア
                    if let Some(damaged) = damaged.get_mut(e) {
                        damaged.clear();
                    }
                }
            }
        }
    }
}
