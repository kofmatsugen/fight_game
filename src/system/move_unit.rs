use amethyst::{
    assets::AssetStorage,
    core::ecs::{Join, Read, ReadStorage, System, WriteStorage},
};
use amethyst_sprite_studio::{
    components::{AnimationTime, PlayAnimationKey},
    resource::{data::AnimationData, AnimationStore},
    traits::animation_file::AnimationFile,
};
use movement_transform::components::Movement;
use std::marker::PhantomData;

pub struct MoveSystem<T> {
    _animation_file: PhantomData<T>,
}

impl<T> MoveSystem<T> {
    pub fn new() -> Self {
        MoveSystem {
            _animation_file: PhantomData,
        }
    }
}

impl<'s, T> System<'s> for MoveSystem<T>
where
    T: AnimationFile,
{
    type SystemData = (
        Read<'s, AnimationStore<T>>,
        Read<'s, AssetStorage<AnimationData<T>>>,
        ReadStorage<'s, PlayAnimationKey<T>>,
        ReadStorage<'s, AnimationTime>,
        WriteStorage<'s, Movement>,
    );

    fn run(&mut self, (store, storage, keys, times, mut movements): Self::SystemData) {
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("move_unit");
        for (key, time, movement) in (&keys, &times, &mut movements).join() {
            move_transform(key, time, movement, &store, &storage);
        }
    }
}

fn move_transform<T>(
    key: &PlayAnimationKey<T>,
    time: &AnimationTime,
    _movement: &mut Movement,
    animation_store: &AnimationStore<T>,
    sprite_animation_storage: &AssetStorage<AnimationData<T>>,
) -> Option<()>
where
    T: AnimationFile,
{
    let (id, &pack_id, &animation_id) = key.play_key()?;

    let pack = animation_store
        .get_animation_handle(id)
        .and_then(|handle| sprite_animation_storage.get(handle))
        .and_then(|data| data.pack(&pack_id))?;
    let animation = pack.animation(&animation_id)?;

    let current_time = match time {
        AnimationTime::Play { current_time, .. } => *current_time,
        AnimationTime::Stop { stopped_time, .. } => *stopped_time,
    };
    let current_frame = animation.sec_to_frame(current_time);
    let prev_frame = animation.sec_to_frame(current_time);

    if current_frame >= animation.total_frame() {
        return None;
    }

    // ルートのIDは0固定なので0指定
    // todo 固定値のIDはconst化するのもあり
    pack.parts().nth(0).map(|_| {
        // これまで経過したアニメーションフレーム分の処理を行う
        // 直前のアニメーションフレームは前のフレームで処理してるので省く
        for _f in (prev_frame..current_frame + 1).skip(1) {
            // todo 移動処理を書く
        }
    });

    Some(())
}
