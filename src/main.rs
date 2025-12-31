use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
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

    fn display(&self, mut writer: impl Write) -> io::Result<()> {
        writeln!(writer, "File Analysis Report")?;
        writeln!(writer, "====================")?;
        writeln!(writer, "File name: {}", self.file_name)?;
        writeln!(
            writer,
            "File extension: {}",
            self.file_extension
                .as_ref()
                .map_or("none", |ext| ext.as_str())
        )?;
        writeln!(writer, "Total lines: {}", self.total_lines)?;
        writeln!(writer, "Empty lines: {}", self.empty_lines)?;
        writeln!(writer, "Line ending type: {}", self.determine_line_ending_type())?;
        writeln!(writer, "DOS line endings (CRLF): {}", self.dos_endings)?;
        writeln!(writer, "Unix line endings (LF): {}", self.unix_endings)?;
        Ok(())
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

fn run(args: Vec<String>) -> Result<(), String> {
    if args.len() != 2 {
        return Err(format!("Usage: {} <file_path>", args[0]));
    }

    match analyze_file(&args[1]) {
        Ok(stats) => {
            stats.display(&mut std::io::stdout()).unwrap();
            Ok(())
        },
        Err(e) => Err(format!("Error analyzing file: {}", e)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn create_temp_file(name: &str, content: &str) -> String {
        let file_path = format!(".\\{}", name);
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file_path
    }

    #[test]
    fn test_file_stats_new() {
        let stats = FileStats::new("test_file".to_string());
        assert_eq!(stats.total_lines, 0);
        assert_eq!(stats.unix_endings, 0);
        assert_eq!(stats.dos_endings, 0);
        assert_eq!(stats.empty_lines, 0);
        assert_eq!(stats.file_extension, None);
        assert_eq!(stats.file_name, "test_file");
    }

    #[test]
    fn test_file_stats_new_with_extension() {
        let stats = FileStats::new("test_file.txt".to_string());
        assert_eq!(stats.file_extension, Some("txt".to_string()));
        assert_eq!(stats.file_name, "test_file.txt");
    }

    #[test]
    fn test_determine_line_ending_type_dos() {
        let mut stats = FileStats::new("test_file.txt".to_string());
        stats.dos_endings = 10;
        stats.unix_endings = 5;
        assert_eq!(stats.determine_line_ending_type(), "DOS/Windows (CRLF)");
    }

    #[test]
    fn test_determine_line_ending_type_unix() {
        let mut stats = FileStats::new("test_file.txt".to_string());
        stats.dos_endings = 5;
        stats.unix_endings = 10;
        assert_eq!(stats.determine_line_ending_type(), "Unix/Linux (LF)");
    }

    #[test]
    fn test_determine_line_ending_type_mixed() {
        let mut stats = FileStats::new("test_file.txt".to_string());
        stats.dos_endings = 10;
        stats.unix_endings = 10;
        assert_eq!(stats.determine_line_ending_type(), "Mixed line endings");
    }

    #[test]
    fn test_determine_line_ending_type_none() {
        let stats = FileStats::new("test_file.txt".to_string());
        assert_eq!(stats.determine_line_ending_type(), "No line endings detected");
    }

    #[test]
    fn test_analyze_file_empty_file() {
        let file_path = create_temp_file("empty.txt", "");
        let stats = analyze_file(&file_path).unwrap();
        assert_eq!(stats.total_lines, 0);
        assert_eq!(stats.unix_endings, 0);
        assert_eq!(stats.dos_endings, 0);
        assert_eq!(stats.empty_lines, 0);
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_analyze_file_unix_endings() {
        let file_path = create_temp_file("unix.txt", "line1\nline2\n");
        let stats = analyze_file(&file_path).unwrap();
        assert_eq!(stats.total_lines, 2);
        assert_eq!(stats.unix_endings, 2);
        assert_eq!(stats.dos_endings, 0);
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_analyze_file_dos_endings() {
        let file_path = create_temp_file("dos.txt", "line1\r\nline2\r\n");
        let stats = analyze_file(&file_path).unwrap();
        assert_eq!(stats.total_lines, 2);
        assert_eq!(stats.unix_endings, 0);
        assert_eq!(stats.dos_endings, 2);
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_analyze_file_not_found() {
        let result = analyze_file("non_existent_file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_display() {
        let stats = FileStats {
            total_lines: 10,
            unix_endings: 5,
            dos_endings: 5,
            empty_lines: 2,
            file_extension: Some("txt".to_string()),
            file_name: "test.txt".to_string(),
        };
        let mut buffer = Vec::new();
        stats.display(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("File Analysis Report"));
        assert!(output.contains("File name: test.txt"));
        assert!(output.contains("File extension: txt"));
        assert!(output.contains("Total lines: 10"));
        assert!(output.contains("Empty lines: 2"));
        assert!(output.contains("Line ending type: Mixed line endings"));
        assert!(output.contains("DOS line endings (CRLF): 5"));
        assert!(output.contains("Unix line endings (LF): 5"));
    }

    #[test]
    fn test_analyze_file_mac_endings() {
        let file_path = create_temp_file("mac.txt", "line1\rline2\r");
        let stats = analyze_file(&file_path).unwrap();
        assert_eq!(stats.total_lines, 1); 
        assert_eq!(stats.unix_endings, 0);
        assert_eq!(stats.dos_endings, 0);
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_analyze_file_empty_lines() {
        let file_path = create_temp_file("empty_lines.txt", "line1\n\nline3");
        let stats = analyze_file(&file_path).unwrap();
        assert_eq!(stats.total_lines, 3);
        assert_eq!(stats.unix_endings, 2);
        assert_eq!(stats.dos_endings, 0);
        assert_eq!(stats.empty_lines, 1);
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_analyze_file_no_newline_at_end() {
        let file_path = create_temp_file("no_newline.txt", "line1\nline2");
        let stats = analyze_file(&file_path).unwrap();
        assert_eq!(stats.total_lines, 2);
        assert_eq!(stats.unix_endings, 1);
        assert_eq!(stats.dos_endings, 0);
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_run_valid_args() {
        let file_path = create_temp_file("valid.txt", "line1\nline2");
        let args = vec!["mdlt".to_string(), file_path.clone()];
        let result = run(args);
        assert!(result.is_ok());
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_run_invalid_args() {
        let args = vec!["mdlt".to_string()];
        let result = run(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_analyze_file_empty_lines_dos() {
        let file_path = create_temp_file("empty_lines_dos.txt", "line1\r\n\r\nline3");
        let stats = analyze_file(&file_path).unwrap();
        assert_eq!(stats.total_lines, 3);
        assert_eq!(stats.unix_endings, 0);
        assert_eq!(stats.dos_endings, 2);
        assert_eq!(stats.empty_lines, 1);
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_run_error_handling() {
        let args = vec!["mdlt".to_string(), "non_existent_file.txt".to_string()];
        let result = run(args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Error analyzing file: The system cannot find the file specified. (os error 2)"
        );
    }

    #[test]
    fn test_analyze_file_complex_file() {
        let file_path = create_temp_file("complex.txt", "line1\n\r\nline3\r\n\nline5");
        let stats = analyze_file(&file_path).unwrap();
        assert_eq!(stats.total_lines, 5);
        assert_eq!(stats.unix_endings, 2);
        assert_eq!(stats.dos_endings, 2);
        assert_eq!(stats.empty_lines, 2);
        fs::remove_file(file_path).unwrap();
    }
}
