use crate::resource::command::{CommandList, CommandStore};
use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter, RonFormat},
    ecs::{Read, ReadExpect, World, WriteExpect},
};

impl CommandLoad for &mut World {
    fn load_command<F, C>(
        &mut self,
        dir_path: F, // コマンドファイルのあるディレクトリパス指定
        command_name: C,
        progress: &mut ProgressCounter,
    ) where
        F: Into<String>,
        C: Into<String>,
    {
        self.exec(
            |(mut store, loader, storage): (
                WriteExpect<CommandStore>,
                ReadExpect<Loader>,
                Read<AssetStorage<CommandList>>,
            )| {
                let dir_path = dir_path.into();
                let command_name = command_name.into();
                let path = format!("{}/{}.com.ron", dir_path, command_name);
                log::info!("load command: {:?}", path);
                let handle = loader.load(path, RonFormat, progress, &storage);
                store.add_command(&command_name, handle);
            },
        );
    }
}

pub trait CommandLoad {
    fn load_command<F, C>(
        &mut self,
        dir_path: F, // コマンドファイルのあるディレクトリパス指定
        command_name: C,
        progress: &mut ProgressCounter,
    ) where
        F: Into<String>,
        C: Into<String>;
}
