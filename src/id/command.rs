use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Command {
    Back,
    Walk,
    BackDash,
    Dash,
    VerticalJump,
    BackJump,
    FrontJump,
    Crouch,
    BackCrouch,
    FrontCrouch,
    A,
    B,
    C,
    D,
}
