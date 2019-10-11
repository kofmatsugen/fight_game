use crate::paramater::AnimationParam;
use amethyst::{
    assets::AssetStorage,
    core::{
        components::Transform,
        ecs::{Join, Read, ReadStorage, System, WriteStorage},
    },
};
use amethyst_sprite_studio::{
    components::{AnimationTime, PlayAnimationKey},
    iter::AnimationRange,
    resource::AnimationStore,
    traits::AnimationKey,
    SpriteAnimation,
};
use std::marker::PhantomData;

pub struct MoveSystem<K> {
    _key: PhantomData<K>,
}

impl<K> MoveSystem<K> {
    pub fn new() -> Self {
        MoveSystem { _key: PhantomData }
    }
}

impl<'s, K> System<'s> for MoveSystem<K>
where
    K: AnimationKey,
{
    type SystemData = (
        Read<'s, AnimationStore<K, AnimationParam>>,
        Read<'s, AssetStorage<SpriteAnimation<AnimationParam>>>,
        ReadStorage<'s, PlayAnimationKey<K>>,
        ReadStorage<'s, AnimationTime>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (store, storage, keys, times, mut transforms): Self::SystemData) {
        for (key, time, transform) in (&keys, &times, &mut transforms).join() {
            move_transform(key, time, transform, &store, &storage);
        }
    }
}

fn move_transform<K>(
    key: &PlayAnimationKey<K>,
    time: &AnimationTime,
    transform: &mut Transform,
    store: &AnimationStore<K, AnimationParam>,
    storage: &AssetStorage<SpriteAnimation<AnimationParam>>,
) -> Option<()>
where
    K: AnimationKey,
{
    // これまで経過したアニメーションフレーム分の処理を行う
    // 直前のアニメーションフレームは前のフレームで処理してるので省く
    for nodes in AnimationRange::new(
        key.key()?,
        time.prev_time(),
        time.current_time(),
        &store,
        &storage,
    )?
    .skip(1)
    {
        nodes
            .filter_map(|(_, _, key_frame, _)| key_frame.user())
            .for_each(|user| {
                let scale_x = transform.scale()[0];
                let scale_y = transform.scale()[1];
                let [x, y] = user.move_direction();
                transform.append_translation_xyz(-x * scale_x, y * scale_y, 0.0);
            });
    }

    Some(())
}
