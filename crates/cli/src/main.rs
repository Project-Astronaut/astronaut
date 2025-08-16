use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "astronaut")] 
#[command(about = "Astronaut Vector DB CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    
    Serve,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Serve => {
            // For now, just call server's main via spawning a process would be cleaner.
            // Placeholder: print instruction.
            println!("Use: cargo run -p astronaut-server");
        }
    }
    Ok(())
}
