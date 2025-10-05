use anyhow::Result;
use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new UUID
    Uuid {
        /// Copy result to clipboard
        #[arg(short, long)]
        clipboard: bool,
    },
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Uuid { clipboard } => {
            let id = Uuid::new_v4().to_string();
            println!("{id}");
            if clipboard {
                let mut clipboard = arboard::Clipboard::new()?;
                clipboard.set_text(id)?;
            }
        }
    }
    Ok(())
}
