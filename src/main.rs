use std::env;
use std::fs;
use std::io::{self, IsTerminal, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("medisimp - Privacy-first medical text redaction and plain-language glossary CLI\n");
        println!("USAGE:");
        println!("  medisimp <report.txt> [--no-redact] [-o output.md]");
        println!("  cat report.txt | medisimp [--no-redact]");
        process::exit(0);
    }

    let redact_pii = !args.iter().any(|a| a == "--no-redact");

    // Check for optional `-o <output_file>` flag
    let output_path = args
        .iter()
        .position(|a| a == "-o" || a == "--output")
        .and_then(|idx| args.get(idx + 1));

    // Find input file path (first argument that isn't a flag or output target)
    let mut input_file: Option<&String> = None;
    let mut skip_next = false;
    for arg in args.iter().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }
        if arg == "-o" || arg == "--output" {
            skip_next = true;
            continue;
        }
        if !arg.starts_with('-') {
            input_file = Some(arg);
            break;
        }
    }

    let content = if let Some(path) = input_file {
        fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("Error reading '{}': {}", path, err);
            process::exit(1);
        })
    } else if !io::stdin().is_terminal() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap_or_else(|err| {
            eprintln!("Error reading from stdin: {}", err);
            process::exit(1);
        });
        buffer
    } else {
        eprintln!("Error: Provide a file path or pipe text via stdin. Run `medisimp --help` for usage.");
        process::exit(1);
    };

    let simplified = medisimp::simplify_report(&content, redact_pii);

    if let Some(out_path) = output_path {
        fs::write(out_path, &simplified).unwrap_or_else(|err| {
            eprintln!("Error writing to '{}': {}", out_path, err);
            process::exit(1);
        });
        println!("Saved sanitized Markdown report to '{}'", out_path);
    } else {
        println!("{}", simplified);
    }
}
