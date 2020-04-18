use crate::traits::ParamaterFromData;
use amethyst::{
    assets::AssetStorage,
    core::Transform,
    ecs::{
        error::WrongGeneration, Entities, Entity, Join, Read, ReadStorage, System, WriteStorage,
    },
};
use amethyst_aabb::Collisions;
use amethyst_sprite_studio::{
    components::{AnimationTime, PlayAnimationKey},
    resource::{data::AnimationData, AnimationStore},
    traits::animation_file::AnimationFile,
};
use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Debug)]
enum RegisterError<T>
where
    T: AnimationFile,
{
    NotFoundPack(T::PackKey),
    NotFoundAnimation(T::AnimationKey),
    NotSetAnimation,
    EntryError(WrongGeneration),
}

pub struct RegisterColliderSystem<T, P> {
    _animation_file: PhantomData<T>,
    _paramater: PhantomData<P>,
}

impl<T, P> RegisterColliderSystem<T, P> {
    pub fn new() -> Self {
        RegisterColliderSystem {
            _animation_file: PhantomData,
            _paramater: PhantomData,
        }
    }
}

impl<'s, T, P> System<'s> for RegisterColliderSystem<T, P>
where
    T: AnimationFile + std::fmt::Debug,
    P: 'static + Send + Sync + ParamaterFromData<T::UserData>,
{
    type SystemData = (
        Entities<'s>,
        Read<'s, AnimationStore<T>>,
        Read<'s, AssetStorage<AnimationData<T>>>,
        ReadStorage<'s, PlayAnimationKey<T>>,
        ReadStorage<'s, AnimationTime>,
        WriteStorage<'s, Collisions<P>>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (entities, store, storage, keys, times, mut collisions, transforms): Self::SystemData,
    ) {
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("register_collider");
        let start = std::time::Instant::now();
        for (e, key, time, transform) in (&*entities, &keys, &times, &transforms).join() {
            let scale = transform.scale();
            let mut root_scale_transform = Transform::default();
            root_scale_transform.set_scale(*scale);
            match register_collision::<T, _>(
                e,
                key,
                time,
                &root_scale_transform,
                &mut collisions,
                &store,
                &storage,
            ) {
                Ok(()) => {}
                Err(err) => log::error!("{:?}", err),
            }
        }
        let end = start.elapsed();
        log::error!("register_collider = {} ms", end.as_millis());
    }
}

fn register_collision<T, P>(
    e: Entity,
    key: &PlayAnimationKey<T>,
    time: &AnimationTime,
    root_scale_transform: &Transform,
    collisions: &mut WriteStorage<Collisions<P>>,
    animation_store: &Read<AnimationStore<T>>,
    sprite_animation_storage: &Read<AssetStorage<AnimationData<T>>>,
) -> Result<(), RegisterError<T>>
where
    T: AnimationFile + std::fmt::Debug,
    P: 'static + Send + Sync + ParamaterFromData<T::UserData>,
{
    let registered_collision = collisions
        .entry(e)
        .map_err(|err| RegisterError::EntryError(err))?
        .or_insert(Collisions::new());
    // 新規に登録し直すので既存のものは削除
    registered_collision.clear();

    let (id, pack_id, animation_id) = key.play_key().ok_or(RegisterError::NotSetAnimation)?;

    let pack = animation_store
        .get_animation_handle(id)
        .and_then(|handle| sprite_animation_storage.get(handle))
        .and_then(|data| data.pack(pack_id))
        .ok_or(RegisterError::NotFoundPack(*pack_id))?;

    let animation = pack
        .animation(animation_id)
        .ok_or(RegisterError::NotFoundAnimation(*animation_id))?;
    let frame = animation.sec_to_frame(time.current_time());
    let mut global_transforms = BTreeMap::new();
    pack.parts().enumerate().for_each(|(part_id, part)| {
        let parent_id = part.parent_id();
        let hash_key = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            pack_id.hash(&mut hasher);
            animation_id.hash(&mut hasher);
            part_id.hash(&mut hasher);
            hasher.finish()
        };

        // 親の位置を取得してからパーツのローカル座標を反映
        // 後で判定登録のときに座標計算するので基本は 0 座標
        let mut part_transform = parent_id
            .and_then(|parent_id| {
                let hash_key = {
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    pack_id.hash(&mut hasher);
                    animation_id.hash(&mut hasher);
                    parent_id.hash(&mut hasher);
                    hasher.finish()
                };
                global_transforms.get(&hash_key)
            })
            .unwrap_or(root_scale_transform)
            .clone();
        part_transform.concat(&animation.local_transform(part_id, frame));

        let user = animation.user(part_id, frame);
        // キーフレームから collisions に登録するデータを生成するトレイト
        if let Some(data) = P::make_collision_data(user) {
            log::trace!(
                "register {:?}: {:?}",
                (pack_id, part_id),
                part_transform.translation()
            );
            registered_collision.add_aabb(
                (
                    part_transform.translation().x,
                    part_transform.translation().y,
                ),
                part_transform.scale().x,
                part_transform.scale().y,
                data,
            );
        }

        // 今のIDで位置を登録しておき，次の子パーツの座標計算に利用する
        global_transforms.insert(hash_key, part_transform);
    });
    Ok(())
}
