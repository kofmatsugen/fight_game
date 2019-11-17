use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CollisionType {
    Extrusion,
    Blow,
    Projectile,
    Throw,
}
