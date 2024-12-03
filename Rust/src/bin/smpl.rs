use clap::Parser as CliParser;
use piet::smpl::SmplExecutor;
use piet::smpl::smpl_to_stk::SmplToStk;
use std::fs;
use std::io::Read;

#[derive(CliParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long)]
    run: bool,
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let unparsed_file = fs::read_to_string(args.filepath).expect("cannot read file");

    let input = std::io::stdin().bytes().peekable();
    let output = std::io::stdout();

    if args.run {
        SmplExecutor::interpret_from_string(
            unparsed_file.as_str(),
            &mut Some(input),
            &mut Some(output),
        );
    }

    if args.output.is_some() {
        SmplToStk::to_stk(SmplExecutor::new(unparsed_file.as_str()));
        // let img: String =
        //     SmplExecutor::to_stk(
        //         unparsed_file.as_str());

        // let mut file = File::create(args.output.unwrap());
        // file.write_all();
    }

}
