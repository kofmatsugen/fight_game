use crate::traits::ParamaterFromData;
use amethyst::{
    core::timing::Time,
    ecs::{
        error::WrongGeneration, Entities, Entity, Join, Read, ReadStorage, System, WriteStorage,
    },
};
use amethyst_aabb::Collisions;
use amethyst_sprite_studio::{
    components::{AnimationNodes, Node},
    traits::animation_file::AnimationFile,
};
use std::marker::PhantomData;

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
    P: 'static + Send + Sync + ParamaterFromData<'s, T::UserData>,
{
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, AnimationNodes<T::UserData>>,
        WriteStorage<'s, Collisions<P>>,
        P::SystemData,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, nodes, mut collisions, collision_system_data, time): Self::SystemData,
    ) {
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("register_collider");
        for (e, nodes) in (&*entities, &nodes).join() {
            log::trace!(
                "[{} F] node frame = {} F",
                time.frame_number(),
                nodes.play_frame()
            );
            match register_collision::<T, _>(e, nodes, &mut collisions, &collision_system_data) {
                Ok(()) => {}
                Err(err) => log::error!("{:?}", err),
            }
        }
    }
}

fn register_collision<'s, T, P>(
    e: Entity,
    nodes: &AnimationNodes<T::UserData>,
    collisions: &mut WriteStorage<Collisions<P>>,
    collision_system_data: &P::SystemData,
) -> Result<(), WrongGeneration>
where
    T: AnimationFile + std::fmt::Debug,
    P: 'static + Send + Sync + ParamaterFromData<'s, T::UserData>,
{
    let registered_collision = collisions.entry(e)?.or_insert(Collisions::new());
    // 新規に登録し直すので既存のものは削除
    registered_collision.clear();

    for Node {
        transform, user, ..
    } in nodes.nodes().filter(|Node { hide, .. }| *hide == false)
    {
        if let Some(param) = P::make_collision_data(e, user.as_ref(), collision_system_data) {
            let translation = transform.translation();
            let scale = transform.scale();
            log::trace!(
                "\t collision = ({}, {}), [{}, {}]",
                translation.x,
                translation.y,
                scale.x,
                scale.y
            );
            registered_collision.add_aabb((translation.x, translation.y), scale.x, scale.y, param);
        }
    }

    // ノードに付随したインスタンスノードの判定も追加
    for instance in nodes.instance_nodes() {
        register_collision::<T, _>(e, instance, collisions, collision_system_data)?;
    }

    Ok(())
}
