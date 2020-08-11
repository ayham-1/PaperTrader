use serde::{Serialize, Deserialize};

use crate::ds::account::position::Position;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Portfolio {
    pub position_history: Vec<Position>,
    pub open_positions: Vec<Position>
}

impl std::fmt::Display for Portfolio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?}, {:#?})", self.position_history, self.open_positions)
    }
}
