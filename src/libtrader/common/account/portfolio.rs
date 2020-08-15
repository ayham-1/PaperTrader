use serde::{Serialize, Deserialize};

use crate::common::account::position::Position;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Portfolio {
    pub open_positions: Vec<Position>
}

impl std::fmt::Display for Portfolio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.open_positions)
    }
}
