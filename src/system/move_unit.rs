use amethyst::{
    assets::AssetStorage,
    core::{
        components::Transform,
        ecs::{Join, Read, ReadStorage, System, WriteStorage},
    },
};
use amethyst_sprite_studio::{
    components::{AnimationTime, PlayAnimationKey},
    resource::{data::AnimationData, AnimationStore},
    traits::animation_file::AnimationFile,
};
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
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (store, storage, keys, times, mut transforms): Self::SystemData) {
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("move_unit");
        for (key, time, transform) in (&keys, &times, &mut transforms).join() {
            move_transform(key, time, transform, &store, &storage);
        }
    }
}

fn move_transform<T>(
    key: &PlayAnimationKey<T>,
    time: &AnimationTime,
    _transform: &mut Transform,
    animation_store: &AnimationStore<T>,
    sprite_animation_storage: &AssetStorage<AnimationData<T>>,
) -> Option<()>
where
    T: AnimationFile,
{
    let (id, &pack_id, &animation_id) = match (key.file_id(), key.pack_name(), key.animation_name())
    {
        (id, Some(pack), Some(anim)) => Some((id, pack, anim)),
        _ => None,
    }?;

    let pack = animation_store
        .get_animation_handle(id)
        .and_then(|handle| sprite_animation_storage.get(handle))
        .and_then(|data| data.pack(&pack_id))?;
    let animation = pack.animation(&animation_id)?;

    let current_frame = animation.sec_to_frame_loop(time.current_time());
    let prev_frame = animation.sec_to_frame_loop(time.prev_time());

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
