use std::os::unix::process::CommandExt;
use std::process::Command;

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

    if cli.dry_run {
        print_command(&cmd);
        return Ok(());
    }

    Err(HpcrError::ExecFailed(cmd.exec()))
}

fn make_runtime(cfg: &FacilityConfig) -> Box<dyn ContainerRuntime> {
    match cfg.facility.runtime {
        Runtime::Apptainer => Box::new(ApptainerRuntime),
        Runtime::Podman => Box::new(PodmanRuntime),
        Runtime::PodmanHpc => Box::new(PodmanHpcRuntime),
    }
}

fn print_command(cmd: &Command) {
    let program = cmd.get_program().to_string_lossy();
    let args: Vec<_> = cmd
        .get_args()
        .map(|a| shell_quote(&a.to_string_lossy()))
        .collect();

    if args.is_empty() {
        println!("{program}");
        return;
    }

    println!("{program} \\");
    for (i, arg) in args.iter().enumerate() {
        if i == args.len() - 1 {
            println!("  {arg}");
        } else {
            println!("  {arg} \\");
        }
    }
}

fn shell_quote(s: &str) -> String {
    if s.chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | '.' | '/' | ':' | '='))
    {
        s.to_owned()
    } else {
        format!("'{}'", s.replace('\'', "'\\''"))
    }
}
