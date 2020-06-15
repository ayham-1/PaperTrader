use crate::ds::account::position::Position;

#[derive(PartialEq, Debug)]
pub struct Portfolio {
    pub position_history: Vec<Position>,
    pub open_positions: Vec<Position>
}
