use crate::traits::ParamaterFromData;
use amethyst::ecs::{
    error::WrongGeneration, Entities, Entity, Join, ReadStorage, System, WriteStorage,
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
    P: 'static + Send + Sync + ParamaterFromData<T::UserData>,
{
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, AnimationNodes<T::UserData>>,
        WriteStorage<'s, Collisions<P>>,
    );

    fn run(&mut self, (entities, nodes, mut collisions): Self::SystemData) {
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("register_collider");
        for (e, nodes) in (&*entities, &nodes).join() {
            match register_collision::<T, _>(e, nodes, &mut collisions) {
                Ok(()) => {}
                Err(err) => log::error!("{:?}", err),
            }
        }
    }
}

fn register_collision<T, P>(
    e: Entity,
    nodes: &AnimationNodes<T::UserData>,
    collisions: &mut WriteStorage<Collisions<P>>,
) -> Result<(), WrongGeneration>
where
    T: AnimationFile + std::fmt::Debug,
    P: 'static + Send + Sync + ParamaterFromData<T::UserData>,
{
    let registered_collision = collisions.entry(e)?.or_insert(Collisions::new());
    // 新規に登録し直すので既存のものは削除
    registered_collision.clear();

    for Node {
        transform, user, ..
    } in nodes.nodes().filter(|Node { hide, .. }| *hide == false)
    {
        if let Some(param) = P::make_collision_data(user.as_ref()) {
            let translation = transform.translation();
            let scale = transform.scale();
            registered_collision.add_aabb((translation.x, translation.y), scale.x, scale.y, param);
        }
    }

    // ノードに付随したインスタンスノードの判定も追加
    for instance in nodes.instance_nodes() {
        register_collision::<T, _>(e, instance, collisions)?;
    }

    Ok(())
}
