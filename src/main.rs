fn main() {
    init_logger();

    eprintln!(
        "starting {} v{} ({} {} built: {})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        option_env!("BUILD_GIT_HASH").unwrap_or("N/A"),
        option_env!("BUILD_GIT_BRANCH").unwrap_or("N/A"),
        option_env!("BUILD_DATETIME").unwrap_or("N/A")
    );
}


/// Initialize the default logging system
fn init_logger() {
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use std::fs::File;
    use std::io;
    let std_log = tracing_subscriber::fmt::layer().with_writer(io::stderr);

    let registry = tracing_subscriber::registry()
        .with(std_log)
        .with(EnvFilter::from_default_env());

    let log_file = File::create(format!("/var/log/{}.log", env!("CARGO_PKG_NAME")));
    if let Ok(log_file) = log_file {
        let file_log = tracing_subscriber::fmt::layer().with_writer(log_file);
        registry.with(file_log).init();
    } else {
        eprintln!("failed to open log file: {}", log_file.unwrap_err());
        registry.init();
    }
}