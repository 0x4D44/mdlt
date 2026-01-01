# mdlt (Medium Line Terminator)

`mdlt` is a lightweight, high-performance Rust CLI utility designed to analyze text files and provide detailed reports on their line termination patterns. It is particularly useful for developers troubleshooting cross-platform line ending issues (LF vs CRLF).

## Features

- **Line Ending Detection:** Identifies Unix (LF), Windows (CRLF), or Mixed line endings.
- **Line Counting:** Reports total line counts and empty line counts.
- **Metadata:** Displays file name and extension.
- **Fast and Safe:** Built with Rust's safety and performance guarantees.
- **High Test Coverage:** Robust codebase with an extensive suite of unit tests.

## Installation

To build `mdlt` from source, you need to have the Rust toolchain installed.

```bash
# Clone the repository
git clone https://github.com/your-username/mdlt.git
cd mdlt

# Build the release version
cargo build --release

# The executable will be available at ./target/release/mdlt
```

## Usage

Pass the path of the file you wish to analyze as an argument:

```bash
cargo run -- <file_path>
```

### Example Output

```text
File Analysis Report
====================
File name: src/main.rs
File extension: rs
Total lines: 254
Empty lines: 42
Line ending type: Unix/Linux (LF)
DOS line endings (CRLF): 0
Unix line endings (LF): 254
```

## Development

### Running Tests

The project includes a comprehensive set of unit tests covering various edge cases (empty files, mixed endings, missing trailing newlines, etc.).

```bash
cargo test
```

### Project Structure

- `src/main.rs`: Contains the core logic, CLI handling, and unit tests.
- `wrk_docs/`: Technical documentation and coverage improvement plans.

## License

This project is licensed under the MIT License - see the LICENSE file for details (or choice of Apache-2.0).
