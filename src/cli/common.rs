use clap::Args;

#[derive(Args, Debug)]
pub struct CommonArgs {
    #[arg(long, help = "Inject facility MPI bind mounts and environment variables")]
    pub mpi: bool,

    #[arg(long, value_name = "SRC:DST", help = "Add a bind mount (repeatable)")]
    pub bind: Vec<String>,

    #[arg(
        long,
        value_name = "KEY=VALUE",
        help = "Set an environment variable (repeatable)"
    )]
    pub env: Vec<String>,
}
