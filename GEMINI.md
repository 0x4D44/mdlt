# mdlt - File Line Ending Analyzer

## Project Overview
`mdlt` is a Rust command-line interface (CLI) utility designed to analyze text files and report statistics regarding their line endings. It helps identify whether a file uses Unix (LF), DOS/Windows (CRLF), or mixed line endings, alongside other metrics like total line count and empty lines.

**Key Features:**
*   Detects Unix (LF) and DOS (CRLF) line endings.
*   Identifies mixed line endings.
*   Counts total lines and empty lines.
*   Reports file extension.

## Building and Running

This project uses `cargo` for dependency management and building.

**Build:**
```bash
cargo build
```

**Run:**
To run the tool against a file:
```bash
cargo run -- <path_to_file>
```
*Example:* `cargo run -- src/main.rs`

**Testing:**
The project maintains a high code coverage standard (targeting 98%+). Tests are located within `src/main.rs` in the `tests` module.
```bash
cargo test
```

## Development Conventions

*   **Architecture:** The project is a single-file application (`src/main.rs`).
    *   `FileStats` struct: Holds the analysis data.
    *   `analyze_file` function: Core logic for reading and parsing files.
    *   `run` function: Entry point wrapper for argument parsing and error handling, facilitating testability.
*   **Testing:** Unit tests are co-located in `src/main.rs` under the `#[cfg(test)] mod tests` module.
*   **Documentation:** Project documentation and improvement plans are stored in the `wrk_docs/` directory.
