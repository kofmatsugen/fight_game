use crate::components::{Damaged, HitInfo, Knockback};
use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, Read, System, WriteStorage},
};
use amethyst_sprite_studio::{components::AnimationTime, traits::animation_file::AnimationFile};
use std::marker::PhantomData;

// ヒットストップの計算基準とするfps
const HIT_STOP_FPS: f32 = 60.;

// ヒット情報を適用する
pub struct ApplyHitInfoSystem<T> {
    _translation: PhantomData<T>,
}

impl<T> ApplyHitInfoSystem<T> {
    pub fn new() -> Self {
        ApplyHitInfoSystem {
            _translation: PhantomData,
        }
    }
}

impl<'s, T> System<'s> for ApplyHitInfoSystem<T>
where
    T: AnimationFile,
{
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, HitInfo<T>>,
        WriteStorage<'s, AnimationTime>,
        WriteStorage<'s, Damaged<T>>,
        WriteStorage<'s, Knockback>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, mut hits, mut times, mut damaged, mut knockback, t): Self::SystemData,
    ) {
        for (e, hit, time) in (&*entities, &mut hits, &mut times).join() {
            // ヒットストップ適用
            if let Some(hitstop_time) = hit.hitstop {
                log::info!(
                    "[{} F] apply hitstop = {} F",
                    t.frame_number(),
                    hitstop_time
                );
                let hitstop_time = hitstop_time as f32 / HIT_STOP_FPS;
                time.stop(hitstop_time);
            }

            // ダメージ判定追加
            if hit.damage_collision_ids.len() > 0 {
                if let Ok(entry) = damaged.entry(e) {
                    let damaged = entry.or_insert(Damaged::new());

                    for id in &hit.damage_collision_ids {
                        log::info!("[{} F] add id => {:?}", t.frame_number(), id);
                        damaged.add_id(*id);
                    }
                }
            }

            // ノックバック時間適用
            if let Some(knockback_time) = hit.knockback {
                if let Ok(entry) = knockback.entry(e) {
                    log::info!(
                        "[{} F] apply knockback = {} F",
                        t.frame_number(),
                        knockback_time
                    );
                    let knockback = entry.or_insert(Knockback::new());
                    let knockback_time = knockback_time as f32 / HIT_STOP_FPS;
                    knockback.set_knockback(knockback_time);
                }
            }

            // ヒット情報のリセット
            *hit = HitInfo::default();
        }
    }
}
