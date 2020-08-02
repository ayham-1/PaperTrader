/// Simple copy-pasta for checking for path existance.
///
/// Used in gen_log() and libtrader_init_log(). Checks if path is exists or not.
pub fn path_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}
