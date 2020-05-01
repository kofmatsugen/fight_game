use crate::{
    system::{direction::DirectionSystem, extrude::ExtrudeSystem, move_unit::MoveSystem},
    traits::{ExtrudeFilter, ParamaterFromData},
};
use amethyst::{
    core::SystemBundle,
    ecs::{DispatcherBuilder, World},
};
use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use std::marker::PhantomData;

// パラメータをもとに座標の移動処理を行うバンドル
pub struct FightTransformBundle<T, P> {
    _animation_file: PhantomData<T>,
    _paramater: PhantomData<P>,
}

impl<T, P> FightTransformBundle<T, P> {
    pub fn new() -> Self {
        FightTransformBundle {
            _animation_file: PhantomData,
            _paramater: PhantomData,
        }
    }
}

impl<'a, 'b, T, P> SystemBundle<'a, 'b> for FightTransformBundle<T, P>
where
    T: AnimationFile + std::fmt::Debug,
    P: 'static
        + Send
        + Sync
        + for<'c> ParamaterFromData<'c, T::UserData>
        + for<'c> ExtrudeFilter<'c>,
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        log::info!("fight transform bundle build");

        builder.add(MoveSystem::<T>::new(), "animation_move_system", &[]);

        builder.add(DirectionSystem::new(), "direction_system", &[]);

        builder.add(ExtrudeSystem::<P>::new(), "extrude_system", &[]);
        Ok(())
    }
}
