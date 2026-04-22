use serde::Deserialize;

use crate::error::HpcrError;

#[derive(Debug, Deserialize)]
pub struct LocalConfig {
    pub facility: String,
}

pub fn load_local_config() -> Result<LocalConfig, HpcrError> {
    let path = dirs::config_dir()
        .ok_or(HpcrError::LocalConfigNotFound)?
        .join("hpcr")
        .join("local.toml");
    if !path.exists() {
        return Err(HpcrError::LocalConfigNotFound);
    }
    let content = std::fs::read_to_string(&path).map_err(HpcrError::LocalConfigRead)?;
    toml::from_str(&content).map_err(Into::into)
}
