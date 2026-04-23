use std::process::Command;

use crate::runtime::podman::{build_exec_args_for, build_run_args_for};
use crate::runtime::{ContainerRuntime, ExecSpec, RunSpec};

pub struct PodmanHpcRuntime;

impl ContainerRuntime for PodmanHpcRuntime {
    fn build_run_command(&self, spec: &RunSpec) -> Command {
        let (binary, args) = build_run_args_for("podman-hpc", spec);
        let mut cmd = Command::new(binary);
        for arg in args {
            cmd.arg(arg);
        }
        cmd
    }

    fn build_exec_command(&self, spec: &ExecSpec) -> Command {
        let (binary, args) = build_exec_args_for("podman-hpc", spec);
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
    use crate::runtime::RunSpec;

    #[test]
    fn uses_podman_hpc_binary() {
        let spec = RunSpec {
            image: "img".to_owned(),
            binds: vec![],
            envs: vec![],
            flags: vec![],
            passthrough_args: vec![],
        };
        let cmd = PodmanHpcRuntime.build_run_command(&spec);
        assert_eq!(cmd.get_program(), "podman-hpc");
    }
}
