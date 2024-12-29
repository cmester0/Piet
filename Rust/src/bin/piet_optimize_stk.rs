use clap::Parser;
use piet::optimize_stk::StackOptimizer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    limit: usize,
    #[arg(short, long)]
    stack: bool,
}

fn main() {
    let args = Args::parse();
    let mut optimizer = StackOptimizer::new();

    for i in 0..args.limit {
        if args.stack {
            println!("{}: {:?}", i, optimizer.optimize_stack(vec![i.into()]))
        } else {
            println!("{}: {:?}", i, optimizer.optimize_number(i))
        }
    }
}
