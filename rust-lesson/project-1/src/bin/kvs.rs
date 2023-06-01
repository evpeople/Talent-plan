use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Get a value
    Get {
        // key to be get
        key: String,
    },
    // Set a value
    Set {
        // key
        key: String,
        // value
        value: String,
    },
    // Remove a value
    Rm {
        // key to be remove
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Get { key: value }) => {
            eprintln!("unimplemented");
            std::process::exit(2);
        }
        Some(Commands::Set { key, value }) => {
            eprintln!("unimplemented");
            std::process::exit(2);
        }
        Some(Commands::Rm { key }) => {
            eprintln!("unimplemented");
            std::process::exit(2);
        }
        None => {
            std::process::exit(2);
        }
    }

    // Continued program logic goes here...
}
