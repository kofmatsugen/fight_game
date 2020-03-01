#[cfg(feature = "debug")]
use crate::system::debug;
use crate::{
    input::FightInput,
    paramater::AnimationParam,
    resource::command::CommandList,
    system::{move_unit::MoveSystem, register_collider::RegisterColliderSystem},
    traits::{CollisionData, CollisionFromData, ParamaterFromData},
};
use amethyst::{
    assets::Processor,
    core::{SystemBundle, Transform},
    ecs::{DispatcherBuilder, World},
};
use amethyst_sprite_studio::traits::{AnimationKey, FileId};
use input_handle::{system::InputHandleSystem, traits::InputParser};
use std::marker::PhantomData;

pub struct FightGameBundle<ID, P, A, C, T> {
    _file_id: PhantomData<ID>,
    _pack_key: PhantomData<P>,
    _animation_key: PhantomData<A>,
    _paramater: PhantomData<C>,
    _collision: PhantomData<T>,
}

impl<ID, P, A, C, T> FightGameBundle<ID, P, A, C, T> {
    pub fn new() -> Self {
        FightGameBundle {
            _file_id: PhantomData,
            _pack_key: PhantomData,
            _animation_key: PhantomData,
            _paramater: PhantomData,
            _collision: PhantomData,
        }
    }
}

impl<'a, 'b, ID, P, A, C, T> SystemBundle<'a, 'b> for FightGameBundle<ID, P, A, C, T>
where
    ID: FileId,
    P: AnimationKey,
    A: AnimationKey,
    C: 'static + Send + Sync + CollisionData + CollisionFromData<Transform> + std::fmt::Debug,
    T: 'static + Send + Sync + ParamaterFromData<AnimationParam>,
{
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        log::info!("fight game bundle build");
        builder.add(
            Processor::<CommandList>::new(),
            "command_list_processor",
            &[],
        );

        world.insert(amethyst::shrev::EventChannel::<
            <FightInput as InputParser>::Event,
        >::default());
        builder.add(MoveSystem::<ID, P, A>::new(), "animation_move_system", &[]);

        builder.add(
            InputHandleSystem::<FightInput>::new(),
            "fight_input_system",
            &[],
        );

        builder.add(
            RegisterColliderSystem::<ID, P, A, C, T>::new(),
            "register_collider_system",
            &[],
        );

        #[cfg(feature = "debug")]
        builder.add(
            debug::input::InputDebugSystem::new(world),
            "input_debug_system",
            &[],
        );

        Ok(())
    }
}
