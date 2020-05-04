use crate::{
    system::direction::DirectionSystem,
    traits::{ExtrudeFilter, ParamaterFromData, UpdateHitInfo},
};
use amethyst::{
    core::SystemBundle,
    ecs::{DispatcherBuilder, World},
};
use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use std::marker::PhantomData;

// パラメータをもとに座標の移動処理を行うバンドル
pub struct FightTransformBundle<T, P, H> {
    _animation_file: PhantomData<T>,
    _paramater: PhantomData<P>,
    _hit_info: PhantomData<H>,
}

impl<T, P, H> FightTransformBundle<T, P, H> {
    pub fn new() -> Self {
        FightTransformBundle {
            _animation_file: PhantomData,
            _paramater: PhantomData,
            _hit_info: PhantomData,
        }
    }
}

impl<'a, 'b, T, P, H> SystemBundle<'a, 'b> for FightTransformBundle<T, P, H>
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
        log::info!("fight transform bundle build");

        builder.add(DirectionSystem::new(), "direction_system", &[]);

        Ok(())
    }
}
