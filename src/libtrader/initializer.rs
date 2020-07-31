#[allow(unused_imports)]
use crate::misc::gen_log::gen_log;
use crate::misc::path_exists::path_exists;
use crate::db::initializer::db_init;
use crate::ds::server::global_state::GlobalState;

pub fn libtrader_init_log() -> Result<(), String> {
    info!("Started Logger.");
    #[cfg(not(debug_assertions))]
    gen_log();

    #[cfg(debug_assertions)]
    {
        use simplelog::*;
        use std::fs::File;

        if !path_exists("log") {
            match std::fs::create_dir("log") {
                Ok(()) => {},
                Err(err) => panic!("GEN_LOG_FAILED_DIR_CREATION: {}", err)
            };
        }
        CombinedLogger::init(vec![
                             #[cfg(debug_assertions)]
                             TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed),
                             #[cfg(not(debug_assertions))]
                             TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
                             WriteLogger::new(LevelFilter::Info, Config::default(), 
                                              File::create(format!("log/log-{}.txt", 
                                                                   chrono::Utc::now().to_rfc2822())).unwrap())
        ]).unwrap();
    };

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


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_libtrader_log() {
        match libtrader_init_log() {
            Ok(()) => {},
            Err(err) => panic!("TEST_INIT_LOG_FAILED: {}", err)
        }
    }
}
