use clap::Args;

use crate::cli::common::CommonArgs;

#[derive(Args, Debug)]
pub struct RunArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    #[arg(value_name = "IMAGE", help = "Container image to run")]
    pub image: String,

    #[arg(
        trailing_var_arg = true,
        allow_hyphen_values = true,
        value_name = "ARGS",
        help = "Arguments passed through to the runtime (use -- to separate runtime flags)"
    )]
    pub args: Vec<String>,
}
