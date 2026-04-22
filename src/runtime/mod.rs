pub mod apptainer;
pub mod podman;
pub mod podman_hpc;

pub use apptainer::ApptainerRuntime;
pub use podman::PodmanRuntime;
pub use podman_hpc::PodmanHpcRuntime;

use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BindMount {
    pub src: PathBuf,
    pub dst: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

pub struct RunSpec {
    pub image: String,
    pub binds: Vec<BindMount>,
    pub envs: Vec<EnvVar>,
    pub passthrough_args: Vec<String>,
}

pub struct ExecSpec {
    pub image: String,
    pub command: Vec<String>,
    pub binds: Vec<BindMount>,
    pub envs: Vec<EnvVar>,
    pub passthrough_args: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Runtime {
    Apptainer,
    Podman,
    PodmanHpc,
}

pub trait ContainerRuntime {
    fn build_run_command(&self, spec: &RunSpec) -> std::process::Command;
    fn build_exec_command(&self, spec: &ExecSpec) -> std::process::Command;
}
