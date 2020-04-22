//! Command-line interface for `print-prep`.

use std::error::Error;
use std::fmt;
use std::str::FromStr;
use structopt::StructOpt;

/// Command-line tool for photo print preparation and other bulk image operations.
///
/// Use `pprep -h`     for help, or
///     `pprep --help` for more detailed help, or
///     `pprep <operation> -h` for help on an operation
///
/// For more documentation and explanation of the algorithm, see the GitHub repository:
///      https://mlange-42.github.io/print-prep/
#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
//#[structopt(name = "chrono-photo command line application")]
pub struct Cli {
    /// List of input files or patterns
    #[structopt(short, long)]
    pub input: Vec<String>,

    /// Output directory path.
    #[structopt(short, long)]
    pub output: String,

    /// Input selection
    #[structopt(subcommand)]
    pub op: Operation,

    /// Debug print parsed command line options
    #[structopt(short, long)]
    pub debug: bool,

    /// Wait for user input after processing
    #[structopt(short, long)]
    pub wait: bool,
}

/// Operations
#[allow(dead_code)]
#[derive(StructOpt, Debug)]
pub enum Operation {
    /// Prepare images for printing.
    Prepare { x: String },
    /// Scale images.
    Scale {
        /// Output image width
        #[structopt(long)]
        width: u32,
        /// Output image height
        #[structopt(long)]
        height: u32,
    },
}

impl FromStr for Cli {
    type Err = ParseCliError;

    /// Parses a string into a Cli.
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let quote_parts: Vec<_> = str.split('"').collect();
        let mut args: Vec<String> = vec![];
        for (i, part) in quote_parts.iter().enumerate() {
            let part = part.trim();
            if i % 2 == 0 {
                args.extend(
                    part.split(' ')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty()),
                );
            } else {
                args.push(part.to_string());
            }
        }
        Ok(Cli::from_iter(args.iter()))
    }
}

/// Error type for failed parsing command line argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCliError(String);

impl Error for ParseCliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for ParseCliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
