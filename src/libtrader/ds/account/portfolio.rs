use postgres_types::{ToSql, FromSql};

use crate::ds::account::position::Position;

#[derive(PartialEq, Debug, ToSql, FromSql)]
pub struct Portfolio {
    pub position_history: Vec<Position>,
    pub open_positions: Vec<Position>
}
