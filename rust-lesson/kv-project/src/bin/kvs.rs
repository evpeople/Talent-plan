use clap::{command, Parser, Subcommand};
use kvs::KvStore;

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
    let mut kv = KvStore::new();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Get { key }) => {
            kv.get(key.to_string()).unwrap_or_else(|_error| {
                println!("Key not found");
                None
            });
            std::process::exit(0);
        }
        Some(Commands::Set { key, value }) => kv
            .set(key.to_string(), value.to_string())
            .unwrap_or_else(|_error| {
                std::process::exit(0);
            }),
        Some(Commands::Rm { key }) => kv.remove(key.to_string()).unwrap_or_else(|_error| {
            println!("Key not found");
            std::process::exit(1);
        }),
        None => {
            std::process::exit(2);
        }
    };

    // Continued program logic goes here...
}
