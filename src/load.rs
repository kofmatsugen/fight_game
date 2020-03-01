use crate::resource::command::{CommandList, CommandListHandle};
use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter, RonFormat},
    ecs::{Read, ReadExpect, World},
};

impl CommandLoad for &mut World {
    fn load_command<F, C>(
        &mut self,
        dir_path: F, // コマンドファイルのあるディレクトリパス指定
        command_name: C,
        progress: &mut ProgressCounter,
    ) -> CommandListHandle
    where
        F: Into<String>,
        C: Into<String>,
    {
        self.exec(
            |(loader, storage): (ReadExpect<Loader>, Read<AssetStorage<CommandList>>)| {
                let dir_path = dir_path.into();
                let command_name = command_name.into();
                let path = format!("{}/{}.com.ron", dir_path, command_name);
                log::info!("load command: {:?}", path);
                loader.load(path, RonFormat, progress, &storage)
            },
        )
    }
}

pub trait CommandLoad {
    fn load_command<F, C>(
        &mut self,
        dir_path: F, // コマンドファイルのあるディレクトリパス指定
        command_name: C,
        progress: &mut ProgressCounter,
    ) -> CommandListHandle
    where
        F: Into<String>,
        C: Into<String>;
}
