pub fn init(level: tracing::Level) -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(level)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
}
