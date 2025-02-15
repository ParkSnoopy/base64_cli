use clap::{Args, Parser, Subcommand};
use clio::*;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Encode or decode given data with base64 encoding",
    long_about = None,
)]
#[command(propagate_version = true)]
pub(crate) struct CliArgs {
    #[clap(subcommand)]
    pub(crate) command: EncodeOption,
}

#[derive(Subcommand, Debug)]

pub(crate) enum EncodeOption {
    /// Encode from plain text to base116 encoded text
    Encode(IOArgs),

    /// Encode from base116 encoded text to plain text
    Decode(IOArgs),
}

#[derive(Args, Debug)]

pub(crate) struct IOArgs {
    /// Input file, use '-' for stdin
    #[clap(long, short, value_parser)]
    pub(crate) r#in: Input,

    /// Output file, use '-' for stdout
    #[clap(long, short, value_parser, default_value = "-")]
    pub(crate) out: Output,

    /// Add trailing null byte as padding '='
    #[clap(long, short, value_parser, default_value_t = false)]
    pub(crate) no_padding: bool,

    /// Replace URL-unsafe chars to URL-safe ('+' => '-' and '/' => '_')
    #[clap(long, short, value_parser, default_value_t = false)]
    pub(crate) urlsafe: bool,
}
