use crate::traits::ParamaterFromData;
use amethyst::ecs::{Entities, Entity, Join, System, WriteStorage};
use amethyst_aabb::Collisions;
use amethyst_sprite_studio::{
    components::{AnimationNodes, BuildRequireData, Node},
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
        BuildRequireData<'s, T>,
        WriteStorage<'s, Collisions<P>>,
        P::SystemData,
    );

    fn run(
        &mut self,
        (
            entities,
            (play_time, key, transforms, tint, storage, store),
            mut collisions,
            collision_system_data,
        ): Self::SystemData,
    ) {
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("register_collider");
        let nodes = (&*entities, &play_time, &key, &transforms, tint.maybe())
            .join()
            .filter_map(|(e, play_time, key, transform, tint)| {
                Some((
                    e,
                    AnimationNodes::<T::UserData>::make_node::<T>(
                        play_time,
                        tint,
                        key.play_key(),
                        transform,
                        transform.global_matrix(),
                        &store,
                        &storage,
                    )?,
                ))
            })
            .collect::<Vec<_>>();
        for (e, nodes) in nodes {
            match collisions.entry(e) {
                Ok(entry) => {
                    let collisions = entry.or_insert(Collisions::new());
                    register_collision::<T, _>(e, &nodes, collisions, &collision_system_data)
                }
                Err(err) => {
                    log::error!("error: {:?}", err);
                }
            }
        }
    }
}

fn register_collision<'s, T, P>(
    e: Entity,
    nodes: &AnimationNodes<T::UserData>,
    collisions: &mut Collisions<P>,
    collision_system_data: &P::SystemData,
) where
    T: AnimationFile + std::fmt::Debug,
    P: 'static + Send + Sync + ParamaterFromData<'s, T::UserData>,
{
    collisions.clear();

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
            collisions.update_aabb((translation.x, translation.y), scale.x, scale.y, param);
        }
    }

    // ノードに付随したインスタンスノードの判定も追加
    for instance in nodes.instance_nodes() {
        register_collision::<T, _>(e, instance, collisions, collision_system_data);
    }
}
