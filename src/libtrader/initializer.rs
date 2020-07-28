use crate::db::config::{*};
use crate::db::init::{db_init, db_gen_connect_str};
use crate::ds::server::global_state::GlobalState;

pub fn libtrader_init() -> Result<GlobalState, String> {
    let mut state: GlobalState = GlobalState::default();

    // Generate db_connect_str.
    db_gen_connect_str(&mut state, DB_USER, DB_PASS);

    // Initialize database.
    match db_init(&mut state) {
        Ok(()) => {
            println!("ran db_init");
        },
        Err(err) => return Err(format!("INIT_DB_FAILED: {}", err))
    }

    Ok(state)
}
