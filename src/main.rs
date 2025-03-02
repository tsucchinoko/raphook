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
    List {
        /// Path to raphook's config file
        #[arg(short, long, default_value = ".")]
        path: String,
    },
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
            println!("Installing hooks in {}", path);

            match cmd::install::install(path) {
                Ok(hooks) => {
                    info!("Successfully installed hooks: {}", hooks.join(", "));
                }
                Err(e) => {
                    error!("❌ Failed to install hooks: {}", e);
                }
            }
        }
        Commands::Run { path, hook_name } => {
            println!("Running hook {}", hook_name);
            // ここにフックの実行ロジックを実装
            match cmd::run::run(path, hook_name) {
                Ok(hooks) => {
                    info!("Successfully run commands: {}", hooks.join(", "));
                }
                Err(e) => {
                    error!("❌ Failed to run commands: {}", e);
                }
            }
        }
        Commands::List { path } => {
            let hooks = raphook::config::Config::load(path);
            if let Err(e) = hooks {
                error!("❌ Failed to load config: {}", e);
                return;
            }

            let mut hook_names = Vec::new();
            for (name, _) in hooks.unwrap().hooks {
                hook_names.push(name.clone());
            }
            println!("Available hooks: {}", hook_names.join(", "));
        }
        Commands::Uninstall { path } => {
            println!("Uninstalling hooks from {}", path);
            match cmd::uninstall::uninstall(path) {
                Ok(hooks) => {
                    info!("Successfully uninstalled hooks: {}", hooks.join(", "));
                }
                Err(e) => {
                    error!("❌ Failed to uninstall hooks: {}", e);
                }
            }
        }
    }
}
