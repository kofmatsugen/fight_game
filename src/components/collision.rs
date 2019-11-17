use crate::traits::CollisionData;
use amethyst::{
    core::Transform,
    ecs::{Component, FlaggedStorage},
};
use amethyst_collision::traits::Collider;
use ncollide2d::{pipeline::CollisionObjectSlabHandle, shape::ShapeHandle};
use std::collections::BTreeMap;

pub(crate) struct CollisionHandler<C, T> {
    pub(crate) collision: C,
    pub(crate) data: T,
    pub(crate) position: Transform,
    pub(crate) handle: Option<CollisionObjectSlabHandle>,
}

// 押し出し判定
pub struct Collisions<ID, C, T>
where
    ID: std::hash::Hash + PartialOrd + Ord,
{
    collision_data: BTreeMap<ID, CollisionHandler<C, T>>,
}

impl<ID, C, T> Collisions<ID, C, T>
where
    ID: std::hash::Hash + PartialOrd + Ord,
    C: CollisionData,
{
    pub fn new() -> Self {
        Collisions {
            collision_data: BTreeMap::new(),
        }
    }

    pub fn update_collision(&mut self, id: ID, data: T, collision: C, position: Transform) {
        match self.collision_data.get_mut(&id) {
            Some(handler) => {
                handler.collision = collision;
                handler.position = position;
            }
            None => {
                self.collision_data.insert(
                    id,
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

impl<ID, C, T> Collider<f32, T> for Collisions<ID, C, T>
where
    ID: 'static + Sync + Send + std::hash::Hash + PartialOrd + Ord,
    C: 'static + Sync + Send + CollisionData,
    T: 'static + Send + Sync,
{
    type Position = Transform;

    fn handles(
        &self,
    ) -> Vec<(
        &Self::Position,
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
                    handle.map(|handle| (position, collision.make_shape(), data, handle))
                },
            )
            .collect()
    }
}

impl<ID, C, T> Component for Collisions<ID, C, T>
where
    ID: 'static + Send + Sync + std::hash::Hash + PartialOrd + Ord,
    C: 'static + Send + Sync,
    T: 'static + Send + Sync,
{
    type Storage = FlaggedStorage<Self>;
}
