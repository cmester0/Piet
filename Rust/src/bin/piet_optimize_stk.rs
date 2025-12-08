use clap::Parser;
use piet::optimize_stk::StackOptimizer;
use piet::rotation_stack::RotStack;

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
            let mut rot_stack = RotStack::new();
            rot_stack.push(i.into());
            println!("{}: {:?}", i, optimizer.optimize_stack(rot_stack))
        } else {
            println!("{}: {:?}", i, optimizer.optimize_number(i.into()))
        }
    }
}
