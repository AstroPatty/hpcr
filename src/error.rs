use thiserror::Error;

#[derive(Debug, Error)]
pub enum HpcrError {
    #[error("local config not found; run `hpcr setup` first")]
    LocalConfigNotFound,

    #[error("failed to read local config: {0}")]
    LocalConfigRead(std::io::Error),

    #[error("failed to parse local config: {0}")]
    LocalConfigParse(#[from] toml::de::Error),

    #[error("unknown facility '{name}'; supported: {available}")]
    UnknownFacility { name: String, available: String },

    #[error("bind conflict: --bind dst '{dst}' is reserved by facility '{facility}'")]
    BindConflict { dst: String, facility: String },

    #[error("env conflict: --env key '{key}' is reserved by facility '{facility}'")]
    EnvConflict { key: String, facility: String },

    #[error("invalid --bind '{input}': expected SRC:DST")]
    InvalidBindFormat { input: String },

    #[error("invalid --env '{input}': expected KEY=VALUE")]
    InvalidEnvFormat { input: String },

    #[error("invalid flag format {input}: expected --valid")]
    InvalidFlagFormat { input: String },

    #[error("facility config parse error for '{facility}': {source}")]
    FacilityConfigParse {
        facility: String,
        source: toml::de::Error,
    },

    #[error("`hpcr exec` requires a COMMAND argument")]
    MissingExecCommand,

    #[error("exec failed: {0}")]
    ExecFailed(std::io::Error),
}
