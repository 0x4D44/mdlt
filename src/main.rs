use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug)]
struct FileStats {
    total_lines: usize,
    unix_endings: usize,
    dos_endings: usize,
    empty_lines: usize,
    file_extension: Option<String>,
    file_name: String,
}

impl FileStats {
    fn new(file_name: String) -> Self {
        FileStats {
            total_lines: 0,
            unix_endings: 0,
            dos_endings: 0,
            empty_lines: 0,
            file_extension: Path::new(&file_name)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(String::from),
            file_name,
        }
    }

    fn determine_line_ending_type(&self) -> &str {
        if self.dos_endings > self.unix_endings {
            "DOS/Windows (CRLF)"
        } else if self.unix_endings > self.dos_endings {
            "Unix/Linux (LF)"
        } else if self.unix_endings == 0 && self.dos_endings == 0 {
            "No line endings detected"
        } else {
            "Mixed line endings"
        }
    }

    fn display(&self) {
        println!("File Analysis Report");
        println!("====================");
        println!("File name: {}", self.file_name);
        println!(
            "File extension: {}",
            self.file_extension
                .as_ref()
                .map_or("none", |ext| ext.as_str())
        );
        println!("Total lines: {}", self.total_lines);
        println!("Empty lines: {}", self.empty_lines);
        println!("Line ending type: {}", self.determine_line_ending_type());
        println!("DOS line endings (CRLF): {}", self.dos_endings);
        println!("Unix line endings (LF): {}", self.unix_endings);
    }
}

fn analyze_file(path: &str) -> io::Result<FileStats> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    let mut stats = FileStats::new(path.to_string());

    let mut current_line = Vec::new();
    let mut i = 0;
    
    while i < contents.len() {
        match contents[i] {
            b'\r' => {
                if i + 1 < contents.len() && contents[i + 1] == b'\n' {
                    // CRLF (DOS) ending
                    stats.dos_endings += 1;
                    stats.total_lines += 1;
                    if current_line.is_empty() {
                        stats.empty_lines += 1;
                    }
                    current_line.clear();
                    i += 2;
                    continue;
                }
                current_line.push(b'\r');
                i += 1;
            }
            b'\n' => {
                // LF (Unix) ending
                stats.unix_endings += 1;
                stats.total_lines += 1;
                if current_line.is_empty() {
                    stats.empty_lines += 1;
                }
                current_line.clear();
                i += 1;
            }
            byte => {
                current_line.push(byte);
                i += 1;
            }
        }
    }

    // Handle last line if it doesn't end with a newline
    if !current_line.is_empty() {
        stats.total_lines += 1;
    }

    Ok(stats)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    match analyze_file(&args[1]) {
        Ok(stats) => stats.display(),
        Err(e) => {
            eprintln!("Error analyzing file: {}", e);
            std::process::exit(1);
        }
    }
}
