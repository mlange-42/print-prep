use indicatif::ProgressBar;
use print_prep::cli::Cli;
use print_prep::util::PathUtil;
use std::error::Error;
use std::time::Duration;
use std::{env, fs};
use structopt::StructOpt;

fn main() {
    let cli = parse_args().unwrap();

    if cli.debug {
        println!("{:#?}", cli);
    }

    let files: Vec<_> = cli
        .input
        .iter()
        .flat_map(|f| PathUtil::list_files(f).unwrap())
        .collect();

    let bar = ProgressBar::new(files.len() as u64);
    for file in files {
        bar.inc(1);

        std::thread::sleep(Duration::from_millis(200));
    }
    bar.finish_and_clear();

    if cli.wait {
        dont_disappear::any_key_to_continue::default();
    }
}

fn parse_args() -> Result<Cli, Box<dyn Error>> {
    let test = false;

    let args: Vec<String> = if test {
        vec!["pprints".to_string(), "...".to_string()]
    } else {
        env::args().collect()
    };

    let args: Cli = if args.len() == 2 && !args[1].starts_with('-') {
        let mut content = fs::read_to_string(&args[1])?;
        content = "ppa ".to_string() + &content.replace("\r\n", " ").replace("\n", " ");
        content.parse()?
    } else {
        Cli::from_args()
    };
    Ok(args)
}
