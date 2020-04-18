#[cfg(feature = "debug")]
use crate::system::debug;
use crate::{
    input::FightInput,
    resource::command::CommandList,
    system::{
        command_activate::CommandActivateSystem, direction::DirectionSystem,
        extrude::ExtrudeSystem, move_unit::MoveSystem, register_collider::RegisterColliderSystem,
    },
    traits::ParamaterFromData,
};
use amethyst::{
    assets::Processor,
    core::SystemBundle,
    ecs::{DispatcherBuilder, World},
};
use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use input_handle::{system::InputHandleSystem, traits::InputParser};
use std::marker::PhantomData;

pub struct FightGameBundle<T, P> {
    _animation_file: PhantomData<T>,
    _paramater: PhantomData<P>,
}

impl<T, P> FightGameBundle<T, P> {
    pub fn new() -> Self {
        FightGameBundle {
            _animation_file: PhantomData,
            _paramater: PhantomData,
        }
    }
}

impl<'a, 'b, T, P> SystemBundle<'a, 'b> for FightGameBundle<T, P>
where
    T: AnimationFile + std::fmt::Debug,
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

        builder.add_thread_local(RegisterColliderSystem::<T, P>::new());

        builder.add(DirectionSystem::new(), "direction_system", &[]);

        builder.add(ExtrudeSystem::<P>::new(), "extrude_system", &[]);

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
