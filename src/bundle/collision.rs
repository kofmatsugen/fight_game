use crate::{
    system::{
        apply_hit_info::ApplyHitInfoSystem, damage_judge::DamageJudgeSystem, extrude::ExtrudeSystem,
    },
    traits::{ExtrudeFilter, ParamaterFromData, UpdateHitInfo},
};
use amethyst::{
    core::SystemBundle,
    ecs::{DispatcherBuilder, World},
};
use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use std::marker::PhantomData;

// 判定処理後に行う処理をまとめたバンドル
pub struct FightCollisionBundle<T, P, H> {
    _animation_file: PhantomData<T>,
    _paramater: PhantomData<P>,
    _hit_info: PhantomData<H>,
}

impl<T, P, H> FightCollisionBundle<T, P, H> {
    pub fn new() -> Self {
        FightCollisionBundle {
            _animation_file: PhantomData,
            _paramater: PhantomData,
            _hit_info: PhantomData,
        }
    }
}

impl<'a, 'b, T, P, H> SystemBundle<'a, 'b> for FightCollisionBundle<T, P, H>
where
    T: AnimationFile + std::fmt::Debug,
    P: 'static
        + Send
        + Sync
        + for<'c> ParamaterFromData<'c, T::UserData>
        + for<'c> ExtrudeFilter<'c>,
    H: for<'c> UpdateHitInfo<'c, Paramater = P>,
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        log::info!("fight collision bundle build");

        // 押出処理
        builder.add(ExtrudeSystem::<P>::new(), "extrude_system", &[]);

        // 判定で起きたことをパラメータへ書き込み処理
        builder.add(DamageJudgeSystem::<H>::new(), "damage_judge_system", &[]);

        builder.add_barrier();

        // 判定を適用
        builder.add(ApplyHitInfoSystem::<T>::new(), "apply_hit_info", &[]);

        Ok(())
    }
}
