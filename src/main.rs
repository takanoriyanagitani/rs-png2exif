use clap::Parser;
use std::io;
use std::process::ExitCode;

use rs_png2exif::stdin2limited2png2stdout;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 1677216)]
    input_img_bytes_max: u64,
}

fn sub() -> Result<(), io::Error> {
    let args = Args::parse();
    stdin2limited2png2stdout(args.input_img_bytes_max)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
