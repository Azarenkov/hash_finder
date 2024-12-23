use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'N')]
    pub n: usize,

    #[arg(short = 'F')]
    pub f: usize,
}

pub fn parse_input() -> (usize, usize) {
    let args = Args::parse();
    (args.n, args.f)
}