use std::os::unix::process::CommandExt;

use clap::Parser;

use hpcr::cli::{Cli, Commands};
use hpcr::command::setup::run_setup;
use hpcr::command::{build_exec_command, build_run_command};
use hpcr::config::{load_facility, load_local_config, FacilityConfig};
use hpcr::error::HpcrError;
use hpcr::runtime::{ApptainerRuntime, ContainerRuntime, PodmanHpcRuntime, PodmanRuntime, Runtime};

fn main() {
    if let Err(e) = run() {
        eprintln!("hpcr: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), HpcrError> {
    let cli = Cli::parse();

    // setup is handled before config loading since it creates the config
    if let Commands::Setup(args) = &cli.command {
        return run_setup(args);
    }

    let local = load_local_config()?;
    let cfg = load_facility(&local.facility)?;
    let rt = make_runtime(&cfg);

    let mut cmd = match &cli.command {
        Commands::Run(args) => build_run_command(&cfg, args, rt.as_ref())?,
        Commands::Exec(args) => build_exec_command(&cfg, args, rt.as_ref())?,
        Commands::Setup(_) => unreachable!(),
    };

    Err(HpcrError::ExecFailed(cmd.exec()))
}

fn make_runtime(cfg: &FacilityConfig) -> Box<dyn ContainerRuntime> {
    match cfg.facility.runtime {
        Runtime::Apptainer => Box::new(ApptainerRuntime),
        Runtime::Podman => Box::new(PodmanRuntime),
        Runtime::PodmanHpc => Box::new(PodmanHpcRuntime),
    }
}
