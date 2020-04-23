use indicatif::ProgressBar;
use print_prep::cli::Cli;
use print_prep::util::{ImageUtil, PathUtil};
use rayon::prelude::*;
use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;
use std::{env, fs};
use structopt::StructOpt;

fn main() {
    let start = Instant::now();

    let cli = parse_args().unwrap();

    if cli.debug {
        println!("{:#?}", cli);
    }

    if let Some(threads) = cli.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .expect("Error building thread pool. Pool already built.");
    }

    let files: Vec<_> = cli
        .input
        .par_iter()
        .flat_map(|f| PathUtil::list_files(f).unwrap())
        .collect();

    let bar = ProgressBar::new(files.len() as u64);
    files.par_iter().for_each(|file: &PathBuf| {
        bar.inc(1);

        let name = file
            .file_name()
            .expect(&format!("Unexpected path format in {:?}", file))
            .to_str()
            .unwrap()
            .to_string();
        let out_path = PathBuf::from(cli.output.replace("*", &name));

        let mut op = cli.op.create_op();
        let input = image::open(file).expect(&format!("Unable to read image {:?}", file));
        let output = op
            .execute(&input)
            .expect(&format!("Unable to process image {:?}", file));

        ImageUtil::save_image(output, &out_path, cli.quality.unwrap_or(95))
            .expect(&format!("Unable to save image to {:?}", out_path));
    });
    bar.finish_and_clear();

    println!("Total time: {:?}", start.elapsed());

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
