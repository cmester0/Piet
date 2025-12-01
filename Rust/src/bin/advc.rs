use clap::Parser as CliParser;
use piet::advc::AdvcExecutor;

#[derive(CliParser, Clone, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    filepath: String,
    #[arg(long)]
    run: bool,
    #[arg(long)]
    output: Option<String>,
    #[arg(long)]
    to_stk: Option<String>,
    #[arg(long)]
    optimize_stk: bool,
    #[arg(long)]
    run_stk: bool,
    #[arg(long)]
    to_piet: Option<String>,
    #[arg(long)]
    run_piet: bool,
    #[arg(long)]
    gui_piet: bool,
    #[arg(long)]
    registers: Option<usize>,
    #[arg(long)]
    steps_per_frame: Option<usize>,
    #[arg(long)]
    start_frame: Option<usize>,
    #[arg(long, action)]
    skip_whitespace: bool,
}

fn main() {
    let args = Args::parse();

    let registers = args.registers.unwrap_or(5);
    let steps_per_frame = args.steps_per_frame.unwrap_or(1);
    let start_frame = args.start_frame.unwrap_or(0);

    let mut advc_executor = AdvcExecutor::new(args.filepath.as_str(), registers);

    advc_executor.handle_advc(
        args.run,
        args.output,
        args.to_stk,
        args.optimize_stk,
        args.run_stk,
        args.to_piet,
        args.run_piet,
        args.gui_piet,
        steps_per_frame,
        start_frame,
        args.skip_whitespace,
    )
}
