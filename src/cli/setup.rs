use clap::Args;

#[derive(Args, Debug)]
pub struct SetupArgs {
    #[arg(
        long,
        value_name = "FACILITY",
        help = "Facility name; skips interactive detection"
    )]
    pub facility: Option<String>,
}
