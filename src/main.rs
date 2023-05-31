use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} input_file output_file", args[0]);
        return;
    }

    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    let input_file = File::open(&input_path).expect("Failed to open input file");
    let reader = BufReader::new(input_file);

    let output_file = File::create(&output_path).expect("Failed to create output file");
    let mut writer = std::io::BufWriter::new(output_file);

    let name_regex = Regex::new(r"^(?P<name>.+?)\s+to\s+Everyone").unwrap();
    let email_regex = Regex::new(r"(?P<email>\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b)").unwrap();

    let mut lines: Vec<String> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        lines.push(line);
    }

    let mut unique_entries = HashSet::new();

    for i in 0..lines.len() {
        if let Some(captures) = email_regex.captures(&lines[i]) {
            let email = captures["email"].to_string();

            let mut name = String::new();
            let mut found_name = false;

            for j in (0..i).rev() {
                if let Some(captures) = name_regex.captures(&lines[j]) {
                    name = captures["name"].to_string();
                    found_name = true;
                    break;
                }
            }

            if found_name {
                unique_entries.insert((name.clone(), email.clone()));
            }
        }
    }

    for (name, email) in unique_entries {
        writeln!(writer, "{}\t{}", name, email).expect("Failed to write to output file");
    }
}
