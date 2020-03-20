use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CollisionType {
    Extrusion,
    Blow,
    Projectile,
    Throw,
}
