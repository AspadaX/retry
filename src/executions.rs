use std::io::BufRead;

#[derive(Debug)]
pub enum RetryErrors {
    ReachedRetryLimit,
    InvalidCommand,
}

impl std::fmt::Display for RetryErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetryErrors::ReachedRetryLimit => {
                write!(f, "Maximum retry attempts reached; operation aborted.")
            }
            RetryErrors::InvalidCommand => write!(f, "Invalid command provided."),
        }
    }
}

impl std::error::Error for RetryErrors {}

/// Struct representing a command to be executed.
#[derive(Clone, Debug)]
pub struct Command {
    pub root: String,
    pub args: Vec<String>,
}

impl Command {
    /// Creates a `Command` struct from a command string.
    pub fn from_string(command_str: &str) -> Result<Self, RetryErrors> {
        let parts: Vec<String> = command_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if parts.is_empty() {
            return Err(RetryErrors::InvalidCommand);
        }

        Ok(Command {
            root: parts[0].clone(),
            args: parts[1..].to_vec(),
        })
    }
}

/// Retries executing the command according to the retry limit or infinitely if `is_infinite` is true.
pub fn retry(
    command: Command,
    mut retries: usize,
    is_infinite: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!(
            "Trying to execute command: {} {}",
            command.root,
            command.args.join(" ")
        );
        match execute_command(&command) {
            Ok(_) => {
                println!("Command executed successfully.");
                return Ok(());
            }
            Err(e) => {
                eprintln!("Command execution failed: {}", e);
                if !is_infinite {
                    if retries == 0 {
                        return Err(Box::new(RetryErrors::ReachedRetryLimit));
                    }
                    retries -= 1;
                    println!("Retrying... Attempts left: {}", retries);
                } else {
                    // Infinite retries
                    println!("Retrying infinitely until success...");
                }
            }
        }
    }
}

/// Executes the given command.
fn execute_command(command: &Command) -> Result<(), Box<dyn std::error::Error>> {
    let mut process = std::process::Command::new(&command.root)
        .args(&command.args)
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    {
        let stdout = process.stdout.as_mut().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to capture standard output.",
            )
        })?;
        let stdout_reader = std::io::BufReader::new(stdout);
        for line in stdout_reader.lines() {
            println!("{}", line?);
        }
    }

    let status = process.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Command exited with status {}", status),
        )))
    }
}