use crate::traits::CollisionData;
use amethyst::{
    core::Transform,
    ecs::{Component, Entity, FlaggedStorage},
};
use amethyst_collision::{paramater::CollisionWorld, traits::Collider, traits::ToIsometry};
use ncollide2d::{math::Isometry, pipeline::CollisionObjectSlabHandle, shape::ShapeHandle};
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::Hasher;
use std::vec::Drain;

pub(crate) struct CollisionHandler<C, T> {
    pub(crate) collision: C,
    pub(crate) data: T,
    pub(crate) position: Transform,
    pub(crate) handle: Option<CollisionObjectSlabHandle>,
}

// 押し出し判定
pub struct Collisions<C, T> {
    collision_data: BTreeMap<u64, CollisionHandler<C, T>>,
    removed_collisions: Vec<CollisionObjectSlabHandle>,
}

impl<C, T> Collisions<C, T> {
    pub fn new() -> Self {
        Collisions {
            collision_data: BTreeMap::new(),
            removed_collisions: vec![],
        }
    }

    pub fn remove_collision<H>(&mut self, id: H) -> Option<(H, CollisionObjectSlabHandle)>
    where
        H: std::hash::Hash + std::fmt::Debug,
    {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        let hash_id = hasher.finish();

        match self.collision_data.remove(&hash_id) {
            Some(CollisionHandler {
                handle: Some(handle),
                ..
            }) => {
                self.removed_collisions.push(handle);
                Some((id, handle))
            }
            _ => None,
        }
    }

    pub fn update_collision<H>(&mut self, id: H, data: T, collision: C, position: Transform)
    where
        H: std::hash::Hash + std::fmt::Debug,
    {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        let hash_id = hasher.finish();

        match self.collision_data.get_mut(&hash_id) {
            Some(handler) => {
                handler.collision = collision;
                handler.position = position;
            }
            None => {
                log::info!("insert id: {:?}", id);
                self.collision_data.insert(
                    hash_id,
                    CollisionHandler {
                        collision,
                        data,
                        position,
                        handle: None,
                    },
                );
            }
        }
    }
}

impl<C, T> Default for Collisions<C, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C, T> Collider for Collisions<C, T>
where
    C: 'static + Sync + Send + CollisionData,
    T: 'static + Send + Sync + Clone + Copy,
{
    type CollisionParamater = T;

    fn registered_handles(
        &self,
    ) -> Vec<(
        Isometry<f32>,
        ShapeHandle<f32>,
        &T,
        CollisionObjectSlabHandle,
    )> {
        self.collision_data
            .iter()
            .filter_map(
                |(
                    _,
                    CollisionHandler {
                        collision,
                        data,
                        position,
                        handle,
                    },
                )| {
                    handle.map(|handle| {
                        (position.to_isometry(), collision.make_shape(), data, handle)
                    })
                },
            )
            .collect()
    }

    // まだ登録されてないものを登録する
    fn register_handles(&mut self, entity: Entity, world: &mut CollisionWorld<f32, T>) {
        self.collision_data.iter_mut().for_each(
            |(
                id,
                CollisionHandler {
                    collision,
                    data,
                    position,
                    handle,
                },
            )| {
                if handle.is_some() {
                    // すでに登録されているものは無視
                    return;
                }

                let registered = world.add_collision(
                    entity,
                    position.to_isometry(),
                    collision.make_shape(),
                    *data,
                    None,
                );
                log::info!("register: {} -> {:?}", id, registered);

                *handle = Some(registered);
            },
        )
    }

    fn take_removed_handles(&mut self) -> Drain<CollisionObjectSlabHandle> {
        self.removed_collisions.drain(..)
    }
}

impl<C, T> Component for Collisions<C, T>
where
    C: 'static + Send + Sync,
    T: 'static + Send + Sync,
{
    type Storage = FlaggedStorage<Self>;
}
