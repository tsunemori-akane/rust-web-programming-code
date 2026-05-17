mod api;
mod enums;
mod structs;
use api::basic_actions::create::create;
use clap::Parser;
use enums::TaskStatus;
use glue::errors::NanoServiceError;
/// cargo run -p to_do_core -- --help
/// when using a cargo run command, the arguments passed in must be separeted from the cargo command with --
#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
struct Args {
    #[arg(short, long)]
    title: String,

    #[arg(short, long)]
    status: String,
}
fn main() -> Result<(), NanoServiceError> {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let args = Args::parse();
    let status_enum = TaskStatus::from_string(&args.status)?;
    let to_do_item = create(&args.title, status_enum)?;
    println!("{}", to_do_item);
    Ok(())
}
