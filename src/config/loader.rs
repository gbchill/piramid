use crate::config::AppConfig;

pub fn load_app_config() -> AppConfig {
    let cfg = AppConfig::from_env();
    if let Err(e) = cfg.validate() {
        eprintln!("Invalid configuration: {}", e);
        std::process::exit(1);
    }
    println!("Using configuration: {:?}", cfg);
    cfg
}
