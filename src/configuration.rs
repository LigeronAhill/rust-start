use crate::{AppError, AppResult};
use config::Value;
use std::collections::HashMap;

pub fn init(file: Option<&str>, env_prefix: Option<&str>) -> AppResult<HashMap<String, Value>> {
    if file.is_none() && env_prefix.is_none() {
        return Err(AppError::WrongInputParameters);
    }
    let mut settings = config::Config::builder();
    if let Some(file) = file {
        settings = settings.add_source(config::File::with_name(file));
    }
    if let Some(env_prefix) = env_prefix {
        settings = settings.add_source(config::Environment::with_prefix(env_prefix));
    }
    let settings = settings.build()?;
    let res = settings.try_deserialize::<HashMap<String, Value>>()?;
    Ok(res)
}
