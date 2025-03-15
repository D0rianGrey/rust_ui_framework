use env_logger::{Builder, Env};

pub fn init_logger() {
    let env = Env::default().filter_or("RUST_LOG", "info");

    Builder::from_env(env)
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .format_module_path(false)
        .init();
}
