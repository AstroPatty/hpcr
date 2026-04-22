use serde::Deserialize;

use crate::error::HpcrError;
use crate::runtime::{BindMount, EnvVar, Runtime};

#[derive(Debug, Deserialize)]
pub struct FacilityConfig {
    pub facility: FacilityMeta,
    #[serde(default)]
    pub binds: Vec<BindMount>,
    #[serde(default)]
    pub envs: Vec<EnvVar>,
    #[serde(default)]
    pub mpi_binds: Vec<BindMount>,
    #[serde(default)]
    pub mpi_envs: Vec<EnvVar>,
}

#[derive(Debug, Deserialize)]
pub struct FacilityMeta {
    pub name: String,
    pub runtime: Runtime,
}

const BUNDLED: &[(&str, &str)] = &[
    (
        "perlmutter",
        include_str!("../../facilities/perlmutter.toml"),
    ),
    ("frontier", include_str!("../../facilities/frontier.toml")),
];

pub fn supported_facilities() -> Vec<&'static str> {
    BUNDLED.iter().map(|(name, _)| *name).collect()
}

pub fn load_facility(name: &str) -> Result<FacilityConfig, HpcrError> {
    let raw = BUNDLED
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, src)| *src)
        .ok_or_else(|| HpcrError::UnknownFacility {
            name: name.to_owned(),
            available: supported_facilities().join(", "),
        })?;
    toml::from_str(raw).map_err(|e| HpcrError::FacilityConfigParse {
        facility: name.to_owned(),
        source: e,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::Runtime;

    #[test]
    fn perlmutter_parses() {
        let cfg = load_facility("perlmutter").unwrap();
        assert_eq!(cfg.facility.name, "perlmutter");
        assert!(matches!(cfg.facility.runtime, Runtime::PodmanHpc));
        assert!(!cfg.mpi_binds.is_empty());
        assert!(!cfg.mpi_envs.is_empty());
    }

    #[test]
    fn frontier_parses() {
        let cfg = load_facility("frontier").unwrap();
        assert_eq!(cfg.facility.name, "frontier");
        assert!(matches!(cfg.facility.runtime, Runtime::Apptainer));
    }

    #[test]
    fn unknown_facility_errors() {
        let err = load_facility("nonexistent").unwrap_err();
        assert!(matches!(err, HpcrError::UnknownFacility { .. }));
    }
}
