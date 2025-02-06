use clap::{Parser, Subcommand};
use std::io::Write;
mod cmd;
mod raphook;

#[derive(Parser, Debug)]
#[command(name = "raphook")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install git hooks
    Install {
        /// Path to git repository
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    /// Run git hooks
    Run {
        /// Path to raphook's config file
        #[arg(short, long, default_value = ".")]
        path: String,
        /// Git hook name
        hook_name: String,
    },
    /// List available hooks
    List,
    /// Remove installed hooks
    Uninstall {
        /// Path to git repository
        #[arg(short, long, default_value = ".")]
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { path } => {
            println!("Installing hooks in {}", path);
            std::io::stdout().flush().unwrap();

            match cmd::install::install(path) {
                Ok(hooks) => {
                    println!("✔️ ({})", hooks.join(", "));
                }
                Err(e) => {
                    println!("❌\nError: {}", e);
                }
            }
        }
        Commands::Run { path, hook_name } => {
            println!("Running hook {}", hook_name);
            // ここにフックの実行ロジックを実装
            match cmd::run::run(path, hook_name) {
                Ok(hooks) => {
                    println!("✔️ Run commands ({})", hooks.join(", "));
                }
                Err(e) => {
                    println!("❌\nError: {}", e);
                }
            }
        }
        Commands::List => {
            println!("Available hooks:");
        }
        Commands::Uninstall { path } => {
            println!("Uninstalling hooks from {}", path);
            match cmd::uninstall::uninstall(path) {
                Ok(hooks) => {
                    println!("✔️ ({})", hooks.join(", "));
                }
                Err(e) => {
                    println!("❌\nError: {}", e);
                }
            }
        }
    }
}
