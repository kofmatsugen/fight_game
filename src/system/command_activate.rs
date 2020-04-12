use crate::{components::ActiveCommand, input::FightInput};
use amethyst::{
    ecs::{Join, Read, ReaderId, System, World, WriteStorage},
    shrev::EventChannel,
};
use input_handle::traits::InputParser;

type Event<'a> = <FightInput as InputParser<'a>>::Event;

pub struct CommandActivateSystem<'a> {
    reader: ReaderId<Event<'a>>,
}

impl<'a> CommandActivateSystem<'a> {
    pub fn new(world: &mut World) -> Self {
        CommandActivateSystem {
            reader: world
                .fetch_mut::<EventChannel<Event<'a>>>()
                .register_reader(),
        }
    }
}

impl<'a, 's> System<'s> for CommandActivateSystem<'a> {
    type SystemData = (
        Read<'s, EventChannel<Event<'a>>>,
        WriteStorage<'s, ActiveCommand>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (channel, mut active_commands) = data;

        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("command_activate");
        // 一旦前フレームで有効化されたコマンドを破棄する
        for active in (&mut active_commands).join() {
            active.clear();
        }

        // 成立したコマンドをセットする
        // あとでアニメーションへの遷移などにつかう
        for &(e, command) in channel.read(&mut self.reader) {
            active_commands
                .get_mut(e)
                .map(|active| active.activate(command));
        }
    }
}
