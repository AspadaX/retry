# Retry Command CLI Tool

A command-line application that retries any system command until it succeeds or a specified retry limit is reached. This tool is useful for automating tasks that may fail intermittently and need to be retried.

## Features

•	Retry Any Command: Execute any command on your system and retry it upon failure.

•	Infinite Retries: Option to retry infinitely until the command succeeds.

•	Retry Limit: Specify the number of times to retry before giving up.

•	Informative Output: Provides detailed output and error messages during execution.

•	Cross-Platform Compatibility: Works on Unix/Linux and Windows systems.

## Installation

### Prerequisites

•	Rust: Ensure you have Rust installed. If not, you can install it from rust-lang.org.

### Install Using Cargo

```bash
cargo install retry
```

### Build from Source

1.	Clone the Repository
```bash
git clone https://github.com/yourusername/retry_command.git
```
2.	Navigate to the Project Directory
```
cd retry_command
```
3.	Build the Project
Build in release mode for an optimized executable:
```bash
cargo build --release
```
The compiled binary will be located in ./target/release/.

## Usage

The application accepts a command to execute and the number of times to retry if it fails. A retry count of 0 means it will retry infinitely until the command succeeds.

### Syntax
```bash
retry_command --command "<command_to_execute>" --times <number_of_retries>
```
### Options

•	-c, --command <COMMAND>: The command to execute (required). Enclose the command in quotes if it contains spaces.

•	-t, --times <TIMES>: Number of times to retry. Use 0 for infinite retries (default is 0).

### Examples

#### Infinite Retries

Retry the command uv python install 3.11 infinitely until it succeeds:
```
./target/release/retry_command --command "uv python install 3.11" --times 0
```
#### Limited Retries

Retry the command uv python install 3.11 up to 5 times:
```
./target/release/retry_command --command "uv python install 3.11" --times 5
```
#### Using Short Flags

Execute a command with short flags for options:
```
./target/release/retry_command -c "echo Hello, World!" -t 3
```
#### Display Help Information
```
./target/release/retry_command --help
```
Output:

A CLI app to retry any command on the system until it succeeds or the retry limit is reached

Usage: retry_command --command <COMMAND> [--times <TIMES>]

Options:
- -c, --command <COMMAND>  The command to execute (in quotes if it contains spaces)
- -t, --times <TIMES>      Number of times to retry (0 means infinite retries) [default: 0]
- -h, --help               Print help information
- -V, --version            Print version information

### Output Explanation

The application provides detailed output during execution:

•	Attempt Notification: Indicates when it is attempting to execute the command.

•	Command Output: Displays the output of the command if any.

•	Failure Notification: Informs if the command execution failed.

•	Retry Information: Displays the number of remaining retry attempts (if not infinite).

•	Success Confirmation: Confirms when the command has executed successfully.

### Sample Outputs

#### Successful Command
```bash
Trying to execute command: echo Hello, World!
Hello, World!
Command executed successfully.
Command completed successfully.
```
#### Failing Command with Limited Retries
```bash
Trying to execute command: some_failing_command
Command execution failed: Command exited with status exit status: 1
Retrying... Attempts left: 2
Trying to execute command: some_failing_command
Command execution failed: Command exited with status exit status: 1
Retrying... Attempts left: 1
Trying to execute command: some_failing_command
Command execution failed: Command exited with status exit status: 1
Failed to execute command: Maximum retry attempts reached; operation aborted.
```
#### Failing Command with Infinite Retries
```bash
Trying to execute command: some_failing_command
Command execution failed: Command exited with status exit status: 1
Retrying infinitely until success...
(Repeats indefinitely until interrupted)
```
## Development

### Project Structure

•	main.rs: The entry point of the application. Handles command-line argument parsing and initiates the retry logic.

•	executions.rs: Contains the logic for command parsing, execution, and retry mechanisms.

### Dependencies

•	Clap: Used for parsing command-line arguments with derive macros.

•	Version: 4.0 or later.

•	Features: derive.

•	Rust Standard Library: Utilizes standard library components for process management and I/O operations.

## Building the Project

To build the project in debug mode (useful during development):
```bash
cargo build
```

## Contributing

Contributions are welcome! If you have ideas for improvements or have found bugs, please open an issue or submit a pull request.

### Steps to Contribute

1.	Fork the Repository
2.	Create a Feature Branch
```bash
git checkout -b feature/your_feature_name
```
3.	Commit Your Changes
```bash
git commit -am 'Add a new feature'
```
4.	Push to the Branch
```bash
git push origin feature/your_feature_name
```
5.	Open a Pull Request

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Contact

For questions, suggestions, or feedback, please contact:

•	Xinyu Bao

•	Email: baoxinyuworks@163.com

•	WeChat: baoxinyu2007

## Acknowledgments

•	Clap Crate: For providing an excellent library for command-line argument parsing.