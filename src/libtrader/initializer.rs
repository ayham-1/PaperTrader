use crate::db::config::{*};
use crate::db::init::db_init;
use crate::ds::server::global_state::GlobalState;

pub fn libtrader_init() -> Result<GlobalState, String> {
    let mut state: GlobalState = GlobalState::default();

    // Generate db_connect_str.
    state.db_connect_str = format!("host={} port={} user={} dbname={} password={}",
                                   DB_HOST, DB_HOST_PORT, DB_USER, DB_NAME, DB_PASS);

    // Initialize database.
    match db_init(&mut state) {
        Ok(()) => {
            println!("ran db_init");
        },
        Err(err) => return Err(format!("INIT_DB_FAILED: {}", err))
    }

    Ok(state)
}
