# Code Coverage Improvement Plan

The goal is to increase the code coverage of the `mdlt` project to at least 98%. This will be achieved by implementing a series of unit tests for the existing codebase.

## Stage 1: Refactor for Testability

The `main` function in `src/main.rs` is currently responsible for both argument parsing and calling the `analyze_file` function. This makes it difficult to test the core logic of the application without running the executable.

1.  **Separate `main` logic:** The logic inside `main` will be extracted into a new function, say `run(args: Vec<String>) -> Result<(), String>`. This function will handle argument parsing and file analysis. The `main` function will then simply call `run` and handle any errors. This will allow us to call `run` from our tests with different arguments.
2.  **Isolate `analyze_file`:** The `analyze_file` function is already quite testable, but it takes a file path as input. For testing, it would be better if it could take a `Read` trait object, so we can pass in-memory buffers instead of creating temporary files. However, to keep things simple for now, we will create temporary files for testing.

## Stage 2: Implement Unit Tests

A new `#[cfg(test)]` module will be added to `src/main.rs`. This module will contain the unit tests.

1.  **Test `FileStats::new`:**
    *   Test that the `FileStats` struct is created with the correct initial values.
    *   Test that the file extension is correctly extracted for various file names.
    *   Test with file names that have no extension.

2.  **Test `FileStats::determine_line_ending_type`:**
    *   Test for DOS, Unix, mixed, and no line endings.

3.  **Test `analyze_file`:**
    *   Test with an empty file.
    *   Test with a file containing only Unix (LF) line endings.
    *   Test with a file containing only DOS (CRLF) line endings.
    *   Test with a file containing mixed line endings.
    *   Test with a file that does not have a newline at the end.
    *   Test with a file containing empty lines.
    *   Test with a non-existent file to ensure `io::Error` is handled correctly.

## Stage 3: Iterative Coverage Improvement

After each set of tests is implemented, the code coverage will be regenerated and analyzed. This iterative process will continue until the 98% coverage target is reached. Any uncovered lines of code will be specifically targeted with new tests.
