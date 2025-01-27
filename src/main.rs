use clap::{Parser, Subcommand};

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
            // ここにインストールのロジックを実装
        }
        Commands::List => {
            println!("Available hooks:");
            // ここに利用可能なフックの一覧表示ロジックを実装
        }
        Commands::Uninstall { path } => {
            println!("Uninstalling hooks from {}", path);
            // ここにアンインストールのロジックを実装
        }
    }
}
