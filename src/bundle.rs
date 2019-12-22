use crate::{
    paramater::AnimationParam,
    system::{move_unit::MoveSystem, register_collider::RegisterColliderSystem},
    traits::{CollisionData, CollisionFromData, ParamaterFromData},
};
use amethyst::{
    core::{SystemBundle, Transform},
    ecs::{DispatcherBuilder, World},
};
use amethyst_sprite_studio::{traits::AnimationKey, types::KeyFrame};

use std::marker::PhantomData;

pub struct FightGameBundle<K, C, T> {
    _animation_name_key: PhantomData<K>,
    _paramater: PhantomData<C>,
    _collision: PhantomData<T>,
}

impl<K, C, T> FightGameBundle<K, C, T> {
    pub fn new() -> Self {
        FightGameBundle {
            _animation_name_key: PhantomData,
            _paramater: PhantomData,
            _collision: PhantomData,
        }
    }
}

impl<'a, 'b, K, C, T> SystemBundle<'a, 'b> for FightGameBundle<K, C, T>
where
    K: AnimationKey,
    C: 'static + Send + Sync + CollisionData + CollisionFromData<Transform> + std::fmt::Debug,
    T: 'static + Send + Sync + ParamaterFromData<KeyFrame<AnimationParam>>,
{
    fn build(self, _: &mut World, builder: &mut DispatcherBuilder) -> Result<(), amethyst::Error> {
        builder.add(MoveSystem::<K>::new(), "animation_move_system", &[]);

        builder.add(
            RegisterColliderSystem::<K, C, T>::new(),
            "register_collider_system",
            &[],
        );

        Ok(())
    }
}
