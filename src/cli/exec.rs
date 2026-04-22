use clap::Args;

use crate::cli::common::CommonArgs;

#[derive(Args, Debug)]
pub struct ExecArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    #[arg(value_name = "IMAGE", help = "Container image to run")]
    pub image: String,

    #[arg(
        trailing_var_arg = true,
        allow_hyphen_values = true,
        value_name = "COMMAND",
        help = "Command to execute in the container followed by its arguments"
    )]
    pub args: Vec<String>,
}
