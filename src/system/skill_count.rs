use crate::components::SkillCount;
use amethyst::ecs::{ReaderId, System, Write, WriteStorage};
use amethyst_sprite_studio::{
    traits::animation_file::AnimationFile,
    types::event::{AnimationEvent, AnimationEventChannel},
};
use std::marker::PhantomData;

// entityごとの行った技の使用回数をカウント
pub struct SkillCountSystem<T>
where
    T: AnimationFile,
{
    _translation: PhantomData<T>,
    reader: Option<ReaderId<AnimationEvent<T>>>,
}

impl<T> SkillCountSystem<T>
where
    T: AnimationFile,
{
    pub fn new() -> Self {
        SkillCountSystem {
            _translation: PhantomData,
            reader: None,
        }
    }
}

impl<'s, T> System<'s> for SkillCountSystem<T>
where
    T: AnimationFile,
{
    type SystemData = (
        Write<'s, AnimationEventChannel<T>>,
        WriteStorage<'s, SkillCount<T>>,
    );

    fn run(&mut self, (mut channel, mut counts): Self::SystemData) {
        if self.reader.is_none() == true {
            self.reader = channel.register_reader().into();
        }

        for event in channel
            .read(self.reader.as_mut().unwrap())
            .filter(|ev| match ev {
                AnimationEvent::ChangeKey { .. } => true,
                _ => false,
            })
        {
            match event {
                &AnimationEvent::ChangeKey {
                    entity,
                    file_id,
                    pack,
                    animation,
                } => {
                    if let Ok(entry) = counts.entry(entity) {
                        let count = entry.or_insert(SkillCount::new());
                        count.increment((file_id, pack, animation));
                    }
                }
                _ => {}
            }
        }
    }
}
