#[cfg(feature = "debug")]
use crate::system::debug;
use crate::{
    input::FightInput,
    resource::command::CommandList,
    system::{
        command_activate::CommandActivateSystem, direction::DirectionSystem,
        extrude::ExtrudeSystem, move_unit::MoveSystem, register_collider::RegisterColliderSystem,
    },
    traits::{CollisionData, CollisionFromData, ParamaterFromData},
};
use amethyst::{
    assets::Processor,
    core::{SystemBundle, Transform},
    ecs::{DispatcherBuilder, World},
};
use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use input_handle::{system::InputHandleSystem, traits::InputParser};
use std::marker::PhantomData;

pub struct FightGameBundle<T, C, P> {
    _animation_file: PhantomData<T>,
    _collision: PhantomData<C>,
    _paramater: PhantomData<P>,
}

impl<T, C, P> FightGameBundle<T, C, P> {
    pub fn new() -> Self {
        FightGameBundle {
            _animation_file: PhantomData,
            _collision: PhantomData,
            _paramater: PhantomData,
        }
    }
}

impl<'a, 'b, T, C, P> SystemBundle<'a, 'b> for FightGameBundle<T, C, P>
where
    T: AnimationFile + std::fmt::Debug,
    C: 'static + Send + Sync + CollisionData + CollisionFromData<Transform> + std::fmt::Debug,
    P: 'static + Send + Sync + ParamaterFromData<T::UserData>,
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

        // コマンドのイベントチャンネル登録
        world.insert(amethyst::shrev::EventChannel::<
            <FightInput as InputParser>::Event,
        >::default());
        world.insert(crate::resource::command::CommandStore::new());
        builder.add(MoveSystem::<T>::new(), "animation_move_system", &[]);

        builder.add(
            InputHandleSystem::<FightInput>::new(),
            "fight_input_system",
            &[],
        );

        builder.add(
            CommandActivateSystem::new(world),
            "command_activate_system",
            &["fight_input_system"],
        );

        builder.add(
            RegisterColliderSystem::<T, C, P>::new(),
            "register_collider_system",
            &[],
        );

        builder.add(DirectionSystem::new(), "direction_system", &[]);

        builder.add(ExtrudeSystem::new(), "extrude_system", &["detect_contact"]);

        #[cfg(feature = "debug")]
        builder.add(
            debug::input::InputDebugSystem::new(world),
            "input_debug_system",
            &[],
        );

        #[cfg(feature = "debug")]
        builder.add(
            debug::command::CommandDebugSystem::new(world),
            "command_debug_system",
            &[],
        );

        Ok(())
    }
}
