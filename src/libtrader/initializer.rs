use crate::db::init::db_init;
use crate::ds::server::global_state::GlobalState;

pub fn libtrader_init() -> Result<GlobalState, String> {
    let state: GlobalState = GlobalState::default();

    match db_init() {
        Ok(client) => {
            println!("{}", client.is_closed());
        },
        Err(err) => return Err(format!("INIT_DB_FAILED: {}", err))
    }

    Ok(state)
}
