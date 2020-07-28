use crate::db::config::{*};
use crate::db::init::{db_init, db_gen_connect_str};
use crate::ds::server::global_state::GlobalState;

pub fn libtrader_init_log() -> Result<(), String> {
    use simplelog::*;
    use std::fs::File;
    CombinedLogger::init(vec![
                         #[cfg(debug_assertions)]
                         TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed),
                         #[cfg(not(debug_assertions))]
                         TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
                         WriteLogger::new(LevelFilter::Info, Config::default(), File::create("log.txt").unwrap())
    ]).unwrap();
    info!("Started Logger.");
    Ok(())
}

pub fn libtrader_init() -> Result<GlobalState, String> {
    let mut state: GlobalState = GlobalState::default();

    // Initialize log.
    match libtrader_init_log() {
        Ok(()) => {},
        Err(err) => panic!("This should not happen!\n{}", err),
    };

    // Generate db_connect_str.
    db_gen_connect_str(&mut state, DB_USER, DB_PASS);

    // Initialize database.
    match db_init(&mut state) {
        Ok(()) => info!("Initialized database."),
        Err(err) => return Err(format!("INIT_DB_FAILED: {}", err))
    }

    Ok(state)
}

