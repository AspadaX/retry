mod executions;

use clap::Parser;

/// A CLI app to retry any command on the system until it succeeds or the retry limit is reached.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The command to execute (in quotes if it contains spaces)
    #[arg(short, long)]
    command: String,

    /// Number of times to retry (0 means infinite retries)
    #[arg(short, long, default_value_t = 0)]
    times: usize,
}

fn main() {
    // Parse command-line arguments using Clap derive macros
    let args = Args::parse();

    let is_infinite = args.times == 0;

    // Parse the command string into a Command struct
    let command = executions::Command::from_string(&args.command)
        .expect("Failed to parse the command");

    // Execute the retry function
    match executions::retry(command, args.times, is_infinite) {
        Ok(_) => println!("Command completed successfully."),
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }
}