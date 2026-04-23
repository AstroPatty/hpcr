use std::ffi::OsString;
use std::process::Command;

use crate::runtime::{bind_args, env_args, flag_args, ContainerRuntime, ExecSpec, RunSpec};

pub struct ApptainerRuntime;

pub(crate) fn build_run_args(spec: &RunSpec) -> Vec<OsString> {
    let mut args = vec![OsString::from("run")];
    args.extend(flag_args(&spec.flags));
    args.extend(bind_args("--bind", &spec.binds));
    args.extend(env_args("--env", &spec.envs));
    args.extend(
        spec.passthrough_args
            .iter()
            .map(|s| OsString::from(s.clone())),
    );
    args.push(OsString::from(spec.image.clone()));
    args
}

pub(crate) fn build_exec_args(spec: &ExecSpec) -> Vec<OsString> {
    let mut args = vec![OsString::from("exec")];
    args.extend(bind_args("--bind", &spec.binds));
    args.extend(env_args("--env", &spec.envs));
    args.extend(
        spec.passthrough_args
            .iter()
            .map(|s| OsString::from(s.clone())),
    );
    args.push(OsString::from(spec.image.clone()));
    for part in &spec.command {
        args.push(OsString::from(part.clone()));
    }
    args
}

impl ContainerRuntime for ApptainerRuntime {
    fn build_run_command(&self, spec: &RunSpec) -> Command {
        let mut cmd = Command::new("apptainer");
        for arg in build_run_args(spec) {
            cmd.arg(arg);
        }
        cmd
    }

    fn build_exec_command(&self, spec: &ExecSpec) -> Command {
        let mut cmd = Command::new("apptainer");
        for arg in build_exec_args(spec) {
            cmd.arg(arg);
        }
        cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use crate::runtime::{BindMount, EnvVar, Flag};

    #[test]
    fn run_args_order() {
        let spec = RunSpec {
            image: "my.sif".to_owned(),
            binds: vec![BindMount {
                src: PathBuf::from("/a"),
                dst: PathBuf::from("/b"),
            }],
            envs: vec![EnvVar {
                key: "X".to_owned(),
                value: "1".to_owned(),
            }],
            flags: vec![Flag {
                long: "--mpi".to_owned(),
            }],
            passthrough_args: vec!["--nv".to_owned()],
        };
        let args: Vec<String> = build_run_args(&spec)
            .into_iter()
            .map(|a| a.into_string().unwrap())
            .collect();
        assert_eq!(args[0], "run");
        let flag_pos = args.iter().position(|a| a == "--mpi").unwrap();
        let bind_pos = args.iter().position(|a| a == "--bind").unwrap();
        let nv_pos = args.iter().position(|a| a == "--nv").unwrap();
        let img_pos = args.iter().position(|a| a == "my.sif").unwrap();
        assert!(flag_pos < bind_pos);
        assert!(bind_pos < nv_pos);
        assert!(nv_pos < img_pos);
        assert_eq!(args[bind_pos + 1], "/a:/b");
    }

    #[test]
    fn exec_args_command_after_image() {
        let spec = ExecSpec {
            image: "my.sif".to_owned(),
            command: vec!["python".to_owned(), "train.py".to_owned()],
            binds: vec![],
            envs: vec![],
            flags: vec![],
            passthrough_args: vec![],
        };
        let args: Vec<String> = build_exec_args(&spec)
            .into_iter()
            .map(|a| a.into_string().unwrap())
            .collect();
        assert_eq!(args[0], "exec");
        let img_pos = args.iter().position(|a| a == "my.sif").unwrap();
        assert_eq!(args[img_pos + 1], "python");
        assert_eq!(args[img_pos + 2], "train.py");
    }
}
