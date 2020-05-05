mod damage_collision_id;
#[cfg(feature = "debug")]
pub mod debug;
mod pair_filter;

pub(crate) use damage_collision_id::DamageCollisionId;
pub(crate) use pair_filter::FightPairFilter;
