use crate::{paramater::Aabb, traits::CollisionData};
use amethyst::core::Transform;

// 判定データを生成する
pub trait CollisionFromData<T>: CollisionData {
    fn make_collision(data: &T) -> Self;
}

impl CollisionFromData<Transform> for Aabb {
    fn make_collision(data: &Transform) -> Aabb {
        let width = data.scale().x;
        let height = data.scale().y;

        Aabb::new(width, height)
    }
}
