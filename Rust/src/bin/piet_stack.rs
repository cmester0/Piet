use clap::Parser as CliParser;
use piet::piet_stack::*;

#[derive(CliParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long)]
    run: bool,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(short, long)]
    optimize: bool,
    #[arg(short, long)]
    run_piet: bool,
    #[arg(short, long)]
    to_piet: Option<String>,
}

fn main() {
    let args = Args::parse();

    let stk_executor = PietStackExecutor::new(args.filepath.as_str());

    stk_executor.handle_stk(
        args.output,
        args.optimize,
        args.run,
        args.to_piet,
        args.run_piet,
    );
}
