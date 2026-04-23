pub mod apptainer;
pub mod podman;
pub mod podman_hpc;

pub use apptainer::ApptainerRuntime;
pub use podman::PodmanRuntime;
pub use podman_hpc::PodmanHpcRuntime;

use std::ffi::OsString;
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

#[derive(Debug, Clone, Deserialize)]
pub struct Flag {
    pub long: String,
}

pub struct RunSpec {
    pub image: String,
    pub flags: Vec<Flag>,
    pub binds: Vec<BindMount>,
    pub envs: Vec<EnvVar>,
    pub passthrough_args: Vec<String>,
}

pub struct ExecSpec {
    pub image: String,
    pub command: Vec<String>,
    pub binds: Vec<BindMount>,
    pub envs: Vec<EnvVar>,
    pub flags: Vec<Flag>,
    pub passthrough_args: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Runtime {
    Apptainer,
    Podman,
    PodmanHpc,
}

pub fn flag_args(flags: &[Flag]) -> Vec<OsString> {
    flags.iter().map(|f| OsString::from(&f.long)).collect()
}

pub fn bind_args(flag: &str, binds: &[BindMount]) -> Vec<OsString> {
    let mut out = Vec::new();
    for b in binds {
        out.push(OsString::from(flag));
        out.push(OsString::from(format!(
            "{}:{}",
            b.src.display(),
            b.dst.display()
        )));
    }
    out
}

pub fn env_args(flag: &str, envs: &[EnvVar]) -> Vec<OsString> {
    let mut out = Vec::new();
    for e in envs {
        out.push(OsString::from(flag));
        out.push(OsString::from(format!("{}={}", e.key, e.value)));
    }
    out
}

pub trait ContainerRuntime {
    fn build_run_command(&self, spec: &RunSpec) -> std::process::Command;
    fn build_exec_command(&self, spec: &ExecSpec) -> std::process::Command;
}
