use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Command {
    A,
    B,
    C,
    D,
    Walk,
    Back,
    Dash,
    BackDash,
    VerticalJump,
    BackJump,
    FrontJump,
    Crouch,
    BackCrouch,
    FrontCrouch,
    Neutral,
}
