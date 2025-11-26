use rust_start::AppResult;

fn main() -> AppResult<()> {
    rust_start::logger::init(tracing::Level::DEBUG)?;
    let settings = rust_start::configuration::init(Some("test-settings.toml"), None)?;
    tracing::debug!("settings initialized successfully");
    if let Some(db_value) = settings.get("database").cloned() {
        let db_settings = db_value.try_deserialize::<DatabaseSettings>()?;
        tracing::debug!("database settings: {:#?}", db_settings);
    }
    Ok(())
}

#[derive(serde::Deserialize, Debug, Clone)]
struct DatabaseSettings {
    #[allow(unused)]
    user: String,
    #[allow(unused)]
    password: String,
    #[allow(unused)]
    host: String,
    #[allow(unused)]
    port: u16,
    #[allow(unused)]
    database: String,
}
