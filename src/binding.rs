use crate::components::PlayerTag;
use amethyst::input::BindingTypes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Axis {
    Up(PlayerTag),
    Right(PlayerTag),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    A(PlayerTag),
    B(PlayerTag),
    C(PlayerTag),
    D(PlayerTag),
}

#[derive(Debug)]
pub struct FightBindings;

impl BindingTypes for FightBindings {
    type Axis = Axis;
    type Action = Action;
}
