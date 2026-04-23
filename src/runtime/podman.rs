use std::ffi::OsString;
use std::process::Command;

use crate::runtime::{bind_args, env_args, flag_args, ContainerRuntime, ExecSpec, RunSpec};

pub struct PodmanRuntime;

pub(crate) fn build_run_args_for(binary: &str, spec: &RunSpec) -> (String, Vec<OsString>) {
    let mut args = vec![OsString::from("run")];
    args.extend(flag_args(&spec.flags));
    args.extend(bind_args("-v", &spec.binds));
    args.extend(env_args("-e", &spec.envs));
    args.extend(
        spec.passthrough_args
            .iter()
            .map(|s| OsString::from(s.clone())),
    );
    args.push(OsString::from(spec.image.clone()));
    (binary.to_owned(), args)
}

pub(crate) fn build_exec_args_for(binary: &str, spec: &ExecSpec) -> (String, Vec<OsString>) {
    let mut args = vec![OsString::from("run")];
    args.extend(flag_args(&spec.flags));
    args.extend(bind_args("-v", &spec.binds));
    args.extend(env_args("-e", &spec.envs));
    args.extend(
        spec.passthrough_args
            .iter()
            .map(|s| OsString::from(s.clone())),
    );
    args.push(OsString::from(spec.image.clone()));
    for part in &spec.command {
        args.push(OsString::from(part.clone()));
    }
    (binary.to_owned(), args)
}

impl ContainerRuntime for PodmanRuntime {
    fn build_run_command(&self, spec: &RunSpec) -> Command {
        let (binary, args) = build_run_args_for("podman", spec);
        let mut cmd = Command::new(binary);
        for arg in args {
            cmd.arg(arg);
        }
        cmd
    }

    fn build_exec_command(&self, spec: &ExecSpec) -> Command {
        let (binary, args) = build_exec_args_for("podman", spec);
        let mut cmd = Command::new(binary);
        for arg in args {
            cmd.arg(arg);
        }
        cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use crate::runtime::{BindMount, EnvVar};

    #[test]
    fn run_uses_v_and_e_flags() {
        let spec = RunSpec {
            image: "img".to_owned(),
            binds: vec![BindMount {
                src: PathBuf::from("/s"),
                dst: PathBuf::from("/d"),
            }],
            envs: vec![EnvVar {
                key: "K".to_owned(),
                value: "V".to_owned(),
            }],
            flags: vec![],
            passthrough_args: vec![],
        };
        let (binary, args) = build_run_args_for("podman", &spec);
        let strs: Vec<String> = args.into_iter().map(|a| a.into_string().unwrap()).collect();
        assert_eq!(binary, "podman");
        assert!(strs.contains(&"-v".to_owned()));
        assert!(strs.contains(&"/s:/d".to_owned()));
        assert!(strs.contains(&"-e".to_owned()));
        assert!(strs.contains(&"K=V".to_owned()));
    }

    #[test]
    fn exec_command_after_image() {
        let spec = ExecSpec {
            image: "img".to_owned(),
            command: vec!["bash".to_owned()],
            binds: vec![],
            envs: vec![],
            flags: vec![],
            passthrough_args: vec![],
        };
        let (_, args) = build_exec_args_for("podman", &spec);
        let strs: Vec<String> = args.into_iter().map(|a| a.into_string().unwrap()).collect();
        let img_pos = strs.iter().position(|s| s == "img").unwrap();
        assert_eq!(strs[img_pos + 1], "bash");
    }
}
