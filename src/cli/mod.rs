pub mod common;
pub mod exec;
pub mod run;
pub mod setup;

use clap::{Parser, Subcommand};

use exec::ExecArgs;
use run::RunArgs;
use setup::SetupArgs;

#[derive(Parser, Debug)]
#[command(name = "hpcr", version, about = "Run containerized jobs on HPC systems")]
pub struct Cli {
    #[arg(
        long,
        global = true,
        help = "Print the container command that would be run without executing it"
    )]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a container image using its built-in entrypoint
    Run(RunArgs),
    /// Run a container image with a user-provided command
    Exec(ExecArgs),
    /// Detect the current facility and write ~/.config/hpcr/local.toml
    Setup(SetupArgs),
}
