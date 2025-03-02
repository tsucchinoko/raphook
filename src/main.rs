use clap::{Parser, Subcommand};
use log::{error, info};
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
    // ロガーの初期化
    raphook::logger::init_logger();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { path } => {
            info!("Installing hooks in {}", path);

            match cmd::install::install(path) {
                Ok(hooks) => {
                    info!("✔️ ({})", hooks.join(", "));
                }
                Err(e) => {
                    error!("❌\nError: {}", e);
                }
            }
        }
        Commands::Run { path, hook_name } => {
            info!("Running hook {}", hook_name);
            // ここにフックの実行ロジックを実装
            match cmd::run::run(path, hook_name) {
                Ok(hooks) => {
                    info!("✔️ Run commands ({})", hooks.join(", "));
                }
                Err(e) => {
                    error!("❌\nError: {}", e);
                }
            }
        }
        Commands::List => {
            info!("Available hooks:");
        }
        Commands::Uninstall { path } => {
            info!("Uninstalling hooks from {}", path);
            match cmd::uninstall::uninstall(path) {
                Ok(hooks) => {
                    info!("✔️ ({})", hooks.join(", "));
                }
                Err(e) => {
                    error!("❌\nError: {}", e);
                }
            }
        }
    }
}
