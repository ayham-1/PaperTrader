use crate::misc::gen_log::gen_log;
use crate::db::initializer::db_init;
use crate::ds::server::global_state::GlobalState;

pub fn libtrader_init_log() -> Result<(), String> {
    info!("Started Logger.");
    gen_log();
    Ok(())
}

pub fn libtrader_init() -> Result<GlobalState, String> {
    let mut state: GlobalState = GlobalState::default();

    // Initialize log.
    match libtrader_init_log() {
        Ok(()) => {},
        Err(err) => panic!("This should not happen!\n{}", err),
    };

    // Initialize database.
    match db_init(&mut state) {
        Ok(()) => info!("Initialized database."),
        Err(err) => return Err(format!("INIT_DB_FAILED: {}", err))
    }

    Ok(state)
}

