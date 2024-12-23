use clap::Parser as CliParser;
use piet::advc::AdvcExecutor;

#[derive(CliParser, Clone, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long)]
    run: bool,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(short, long)]
    to_stk: Option<String>,
    #[arg(short, long)]
    optimize_stk: bool,
    #[arg(short, long)]
    run_stk: bool,
    #[arg(short, long)]
    to_piet: Option<String>,
    #[arg(short, long)]
    run_piet: bool,
    #[arg(short, long)]
    gui_piet: bool,
    #[arg(short, long)]
    registers: Option<usize>,
    #[arg(short, long)]
    steps_per_frame: Option<usize>,
    #[arg(short, long)]
    start_frame: Option<usize>,
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
    )
}
