use clap::{Parser, Subcommand};
use lqf::{parse_lqf};
use std::fs;
use std::process;

#[derive(Parser)]
#[command(name = "lqf")]
#[command(about = "A CLI for parsing and visualizing LQF configuration files")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse an LQF file and display the results
    Parse {
        /// Path to the LQF file to parse
        #[arg(value_name = "FILE")]
        file: String,
        
        /// Output format (json or debug)
        #[arg(short, long, default_value = "debug")]
        format: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { file, format } => {
            if let Err(e) = parse_command(&file, &format) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn parse_command(file_path: &str, format: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the file content
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    // Parse the LQF content
    let parsed = parse_lqf(&content)
        .map_err(|e| format!("Failed to parse LQF file: {}", e))?;

    // Output based on format
    match format.to_lowercase().as_str() {
        "json" => {
            // Convert to JSON-like format (simplified since we don't have serde)
            println!("{{");
            for (section_name, section) in &parsed {
                println!("  \"{}\": {{", section_name);
                for (key, value) in section {
                    let value_str = match value {
                        lqf::Value::String(s) => format!("\"{}\"", s),
                        lqf::Value::Number(n) => n.to_string(),
                        lqf::Value::Boolean(b) => b.to_string(),
                        lqf::Value::Array(arr) => {
                            let items: Vec<String> = arr.iter().map(|v| match v {
                                lqf::Value::String(s) => format!("\"{}\"", s),
                                lqf::Value::Number(n) => n.to_string(),
                                lqf::Value::Boolean(b) => b.to_string(),
                                lqf::Value::Null => "null".to_string(),
                                _ => "...".to_string(),
                            }).collect();
                            format!("[{}]", items.join(", "))
                        }
                        lqf::Value::Null => "null".to_string(),
                    };
                    println!("    \"{}\": {},", key, value_str);
                }
                println!("  }},");
            }
            println!("}}");
        }
        "debug" | _ => {
            println!("Parsed LQF file successfully!");
            println!("Found {} section(s):\n", parsed.len());
            
            for (section_name, section) in &parsed {
                println!("[{}]", section_name);
                for (key, value) in section {
                    let value_str = match value {
                        lqf::Value::String(s) => format!("\"{}\"", s),
                        lqf::Value::Number(n) => {
                            if n.fract() == 0.0 {
                                format!("{}", *n as i64)
                            } else {
                                format!("{}", n)
                            }
                        }
                        lqf::Value::Boolean(b) => b.to_string(),
                        lqf::Value::Array(arr) => {
                            let items: Vec<String> = arr.iter().map(|v| match v {
                                lqf::Value::String(s) => format!("\"{}\"", s),
                                lqf::Value::Number(n) => {
                                    if n.fract() == 0.0 {
                                        format!("{}", *n as i64)
                                    } else {
                                        format!("{}", n)
                                    }
                                }
                                lqf::Value::Boolean(b) => b.to_string(),
                                lqf::Value::Null => "null".to_string(),
                                _ => "...".to_string(),
                            }).collect();
                            format!("[{}]", items.join(", "))
                        }
                        lqf::Value::Null => "null".to_string(),
                    };
                    println!("  {} = {}", key, value_str);
                }
                println!();
            }
        }
    }

    Ok(())
}
