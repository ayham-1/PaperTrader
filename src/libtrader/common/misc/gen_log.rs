use crate::common::misc::path_exists::path_exists;

/// Generates the CombinedLogger for simplelog.rs
///
/// Used in libtrader_init_log() Loggers are generated base on platform/configuration.
/// Linux will use /var/log/papertrader/.
/// macOS will use /var/log/papertrader/.
/// Windows & other OSes will output to a file in the current directory.
pub fn gen_log() {
    /*
     * Linux will use /var/log/papertrader/.
     * macOS will use /var/log/papertrader/.
     * Windows & other oses will output to a file in the current directory.
     * */
    use simplelog::*;
    use std::fs::File;

    match os_type::current_platform().os_type {
        os_type::OSType::Unknown => {
            if !path_exists("log") {
                match std::fs::create_dir("log") {
                    Ok(()) => {}
                    Err(err) => panic!("GEN_LOG_FAILED_DIR_CREATION: {}", err),
                };
            }
            CombinedLogger::init(vec![
                #[cfg(debug_assertions)]
                TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed),
                #[cfg(not(debug_assertions))]
                TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
                WriteLogger::new(
                    LevelFilter::Info,
                    Config::default(),
                    File::create(format!("log/log-{}.txt", chrono::Utc::now().to_rfc2822()))
                        .unwrap(),
                ),
            ])
            .unwrap();
        }
        _ => {
            if !path_exists("/var/log/papertrader/") {
                match std::fs::create_dir("/var/log/papertrader/") {
                    Ok(()) => {}
                    Err(err) => panic!("GEN_LOG_FAILED_DIR_CREATION: {}", err),
                };
            }
            CombinedLogger::init(vec![
                #[cfg(debug_assertions)]
                TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed),
                #[cfg(not(debug_assertions))]
                TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
                WriteLogger::new(
                    LevelFilter::Info,
                    Config::default(),
                    File::create(format!(
                        "/var/log/papertrader/log-{}.txt",
                        chrono::Utc::now().to_rfc2822()
                    ))
                    .unwrap(),
                ),
            ])
            .unwrap();
        }
    };
}
