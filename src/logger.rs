use crate::AppResult;
pub fn init(level: tracing::Level) -> AppResult<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(level)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    tracing::debug!("DEBUG messages allowed");
    tracing::info!("INFO messages allowed");
    tracing::warn!("WARN messages allowed");
    tracing::error!("ERROR messages allowed");
    tracing::info!("logger initialized with level: '{level}");
    Ok(())
}
