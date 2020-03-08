use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command {
    A,
    B,
    C,
    D,
    Walk,
    Dash,
    BackDash,
    VerticalJump,
    BackJump,
    FrontJump,
}
