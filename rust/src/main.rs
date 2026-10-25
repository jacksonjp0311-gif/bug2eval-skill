use clap::{Parser, Subcommand};

/// Rust-first Bug2Eval companion.
/// Single static binary for capturing, validating, and replaying portable eval cases.
#[derive(Parser)]
#[command(name = "bug2eval", about = "Portable deterministic evals for coding agents")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Capture a case from Git refs or directories
    Capture,
    /// Validate before-fails / after-passes contract and checksums
    Validate { case: String },
    /// Run a case against an agent command
    Run { case: String },
    /// Pack a case into a .b2e bundle
    Pack { case: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Capture => {
            println!("capture: not yet implemented — see SKILL.md and the Python reference");
        }
        Commands::Validate { case } => {
            println!("validate {case}: not yet implemented");
        }
        Commands::Run { case } => {
            println!("run {case}: not yet implemented");
        }
        Commands::Pack { case } => {
            println!("pack {case}: not yet implemented");
        }
    }
}
