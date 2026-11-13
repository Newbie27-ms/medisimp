use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: medisimp <path-to-report.txt> [--no-redact]");
        process::exit(1);
    }

    let file_path = &args[1];
    let redact_pii = !args.iter().any(|arg| arg == "--no-redact");

    let content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", file_path, err);
        process::exit(1);
    });

    let simplified = medisimp::simplify_report(&content, redact_pii);
    println!("{}", simplified);
}
