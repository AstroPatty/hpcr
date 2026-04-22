pub mod conflict;
pub mod setup;

use std::path::PathBuf;
use std::process::Command;

use crate::cli::exec::ExecArgs;
use crate::cli::run::RunArgs;
use crate::command::conflict::{check_bind_conflicts, check_env_conflicts};
use crate::config::facility::FacilityConfig;
use crate::error::HpcrError;
use crate::runtime::{BindMount, ContainerRuntime, EnvVar, ExecSpec, RunSpec};

pub fn parse_bind(s: &str) -> Result<BindMount, HpcrError> {
    let (src, dst) = s
        .split_once(':')
        .ok_or_else(|| HpcrError::InvalidBindFormat { input: s.to_owned() })?;
    Ok(BindMount {
        src: PathBuf::from(src),
        dst: PathBuf::from(dst),
    })
}

pub fn parse_env(s: &str) -> Result<EnvVar, HpcrError> {
    let (key, value) = s
        .split_once('=')
        .ok_or_else(|| HpcrError::InvalidEnvFormat { input: s.to_owned() })?;
    Ok(EnvVar {
        key: key.to_owned(),
        value: value.to_owned(),
    })
}

fn expand_facility(cfg: &FacilityConfig, mpi: bool) -> (Vec<BindMount>, Vec<EnvVar>) {
    let mut binds = cfg.binds.clone();
    let mut envs = cfg.envs.clone();
    if mpi {
        binds.extend(cfg.mpi_binds.iter().cloned());
        envs.extend(cfg.mpi_envs.iter().cloned());
    }
    (binds, envs)
}

pub fn build_run_command(
    cfg: &FacilityConfig,
    args: &RunArgs,
    rt: &dyn ContainerRuntime,
) -> Result<Command, HpcrError> {
    let user_binds: Vec<BindMount> = args
        .common
        .bind
        .iter()
        .map(|s| parse_bind(s))
        .collect::<Result<_, _>>()?;
    let user_envs: Vec<EnvVar> = args
        .common
        .env
        .iter()
        .map(|s| parse_env(s))
        .collect::<Result<_, _>>()?;

    let (facility_binds, facility_envs) = expand_facility(cfg, args.common.mpi);

    check_bind_conflicts(&cfg.facility.name, &facility_binds, &user_binds)?;
    check_env_conflicts(&cfg.facility.name, &facility_envs, &user_envs)?;

    let mut all_binds = facility_binds;
    all_binds.extend(user_binds);
    let mut all_envs = facility_envs;
    all_envs.extend(user_envs);

    let spec = RunSpec {
        image: args.image.clone(),
        binds: all_binds,
        envs: all_envs,
        passthrough_args: args.args.clone(),
    };

    Ok(rt.build_run_command(&spec))
}

pub fn build_exec_command(
    cfg: &FacilityConfig,
    args: &ExecArgs,
    rt: &dyn ContainerRuntime,
) -> Result<Command, HpcrError> {
    if args.args.is_empty() {
        return Err(HpcrError::MissingExecCommand);
    }

    let user_binds: Vec<BindMount> = args
        .common
        .bind
        .iter()
        .map(|s| parse_bind(s))
        .collect::<Result<_, _>>()?;
    let user_envs: Vec<EnvVar> = args
        .common
        .env
        .iter()
        .map(|s| parse_env(s))
        .collect::<Result<_, _>>()?;

    let (facility_binds, facility_envs) = expand_facility(cfg, args.common.mpi);

    check_bind_conflicts(&cfg.facility.name, &facility_binds, &user_binds)?;
    check_env_conflicts(&cfg.facility.name, &facility_envs, &user_envs)?;

    let mut all_binds = facility_binds;
    all_binds.extend(user_binds);
    let mut all_envs = facility_envs;
    all_envs.extend(user_envs);

    let spec = ExecSpec {
        image: args.image.clone(),
        command: args.args.clone(),
        binds: all_binds,
        envs: all_envs,
        passthrough_args: vec![],
    };

    Ok(rt.build_exec_command(&spec))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_bind_basic() {
        let b = parse_bind("/src:/dst").unwrap();
        assert_eq!(b.src, PathBuf::from("/src"));
        assert_eq!(b.dst, PathBuf::from("/dst"));
    }

    #[test]
    fn parse_bind_no_colon_errors() {
        assert!(parse_bind("/src_only").is_err());
    }

    #[test]
    fn parse_env_basic() {
        let e = parse_env("KEY=value").unwrap();
        assert_eq!(e.key, "KEY");
        assert_eq!(e.value, "value");
    }

    #[test]
    fn parse_env_equals_in_value() {
        let e = parse_env("KEY=val=ue").unwrap();
        assert_eq!(e.key, "KEY");
        assert_eq!(e.value, "val=ue");
    }

    #[test]
    fn parse_env_no_equals_errors() {
        assert!(parse_env("NOEQUALS").is_err());
    }
}
