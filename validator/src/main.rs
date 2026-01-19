//! Worldview Validator CLI
//!
//! A command-line tool for validating Worldview format (.wvf) files.

use std::env;
use std::path::Path;
use std::process::ExitCode;
use worldview_validator::{validate, validate_file};

fn print_usage(program: &str) {
    eprintln!("Usage: {} <file.wvf>", program);
    eprintln!("       {} --stdin", program);
    eprintln!();
    eprintln!("Validates a Worldview format file for syntactic correctness.");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --stdin    Read from standard input instead of a file");
    eprintln!("  --help     Show this help message");
    eprintln!("  --version  Show version information");
}

fn print_version() {
    eprintln!("worldview-validate {}", env!("CARGO_PKG_VERSION"));
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    if args.len() < 2 {
        print_usage(program);
        return ExitCode::from(1);
    }

    let arg = &args[1];

    if arg == "--help" || arg == "-h" {
        print_usage(program);
        return ExitCode::SUCCESS;
    }

    if arg == "--version" || arg == "-V" {
        print_version();
        return ExitCode::SUCCESS;
    }

    let result = if arg == "--stdin" {
        // Read from stdin
        let mut input = String::new();
        if let Err(e) = std::io::Read::read_to_string(&mut std::io::stdin(), &mut input) {
            eprintln!("Error reading from stdin: {}", e);
            return ExitCode::from(1);
        }
        validate(&input)
    } else {
        // Read from file
        let path = Path::new(arg);

        if !path.exists() {
            eprintln!("Error: File '{}' not found", arg);
            return ExitCode::from(1);
        }

        // Check file extension
        if let Some(ext) = path.extension() {
            if ext.to_ascii_lowercase() != "wvf" {
                eprintln!("Warning: File does not have .wvf extension");
            }
        }

        match validate_file(path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", arg, e);
                return ExitCode::from(1);
            }
        }
    };

    if result.is_valid() {
        if result.has_warnings() {
            println!("Valid Worldview document with {} warning(s):", result.warnings.len());
            for warning in &result.warnings {
                println!("  {}", warning);
            }
        } else {
            println!("Valid Worldview document");
        }
        ExitCode::SUCCESS
    } else {
        eprintln!("Invalid Worldview document ({} error(s)):", result.errors.len());
        for error in &result.errors {
            eprintln!("  {}", error);
        }
        if result.has_warnings() {
            eprintln!("Additionally, {} warning(s):", result.warnings.len());
            for warning in &result.warnings {
                eprintln!("  {}", warning);
            }
        }
        ExitCode::from(1)
    }
}
