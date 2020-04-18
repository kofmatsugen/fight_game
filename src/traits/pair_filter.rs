use crate::paramater::CollisionParamater;
use amethyst_aabb::traits::PairFilter;

impl<'s> PairFilter<'s> for CollisionParamater {
    type SystemData = ();
}
