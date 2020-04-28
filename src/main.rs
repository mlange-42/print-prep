use print_prep::cli::Cli;
use print_prep::util::PathUtil;
use print_prep::ErrorAbort;
use rayon::prelude::*;
use std::error::Error;
use std::process::exit;
use std::time::Instant;
use std::{env, fs};
use structopt::StructOpt;

fn main() {
    let start = Instant::now();

    let cli = parse_args().unwrap();

    if cli.debug {
        eprintln!("{:#?}", cli);
    }

    if let Some(threads) = cli.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .exit("Error building thread pool. Pool already built.");
    }

    let files: Vec<_> = cli
        .input
        .par_iter()
        .flat_map(|f| PathUtil::list_files(f).unwrap())
        .collect();

    let op = cli.op.get_op();
    match op.execute(&files[..]) {
        Ok(()) => {}
        Err(e) => {
            exit_on_error(&format!("Error processing images: {:?}", e));
            unreachable!()
        }
    };

    eprintln!("Success! Total time: {:?}", start.elapsed());

    if cli.wait {
        dont_disappear::any_key_to_continue::default();
    }
}

fn parse_args() -> Result<Cli, Box<dyn Error>> {
    let test = false;

    let args: Vec<String> = if test {
        vec![
            "pprep".to_string(),
            "cmd_examples_dev/prep-exif.pprep".to_string(),
            //"cmd_examples_dev/list.pprep".to_string(),
        ]
    } else {
        env::args().collect()
    };

    let args: Cli = if args.len() == 2 && !args[1].starts_with('-') {
        let mut content = fs::read_to_string(&args[1])?;
        println!("{:?}", content);
        content = "ppa ".to_string() + &content.replace("\r\n", " ").replace("\n", " ");
        content.parse()?
    } else {
        Cli::from_args()
    };
    Ok(args)
}

fn exit_on_error(message: &str) {
    eprintln!("Terminated with ERROR:");
    eprintln!("{}", message);
    exit(1);
}
