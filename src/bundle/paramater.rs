#[cfg(feature = "debug")]
use crate::system::debug;
use crate::{
    input::FightInput,
    resource::command::CommandList,
    system::{
        apply_hit_info::ApplyHitInfoSystem, command_activate::CommandActivateSystem,
        damage_judge::DamageJudgeSystem, register_collider::RegisterColliderSystem,
    },
    traits::{ExtrudeFilter, ParamaterFromData, UpdateHitInfo},
};
use amethyst::{
    assets::Processor,
    core::SystemBundle,
    ecs::{DispatcherBuilder, World},
};
use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use input_handle::{system::InputHandleSystem, traits::InputParser};
use std::marker::PhantomData;

// パラメータのセット，登録を行うバンドル
pub struct FightParamaterBundle<T, P, H> {
    _animation_file: PhantomData<T>,
    _paramater: PhantomData<P>,
    _hit_info: PhantomData<H>,
}

impl<T, P, H> FightParamaterBundle<T, P, H> {
    pub fn new() -> Self {
        FightParamaterBundle {
            _animation_file: PhantomData,
            _paramater: PhantomData,
            _hit_info: PhantomData,
        }
    }
}

impl<'a, 'b, T, P, H> SystemBundle<'a, 'b> for FightParamaterBundle<T, P, H>
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
        world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        log::info!("fight paramater bundle build");
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

        // 判定で起きたことをパラメータへ書き込み処理
        builder.add(DamageJudgeSystem::<H>::new(), "damage_judge_system", &[]);

        // 判定を適用
        builder.add(ApplyHitInfoSystem::new(), "apply_hit_info", &[]);

        // 判定登録
        builder.add(
            RegisterColliderSystem::<T, P>::new(),
            "register_collider",
            &[],
        );

        Ok(())
    }
}
