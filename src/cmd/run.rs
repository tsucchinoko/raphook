use std::io;

use crate::raphook;
use std::process::Command;

fn execute_command(command: &str) -> io::Result<()> {
    let status = if cfg!(windows) {
        Command::new("cmd").args(["/C", command]).status()?
    } else {
        Command::new("sh").args(["-c", command]).status()?
    };

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command failed: {}", command),
        ));
    }

    Ok(())
}

pub fn run(path: &str, hook_name: &str) -> io::Result<Vec<String>> {
    // 設定ファイルの読み込み
    let config = raphook::config::Config::load(path)?;
    // 設定ファイルからフックのコマンドを取得
    let hook = config.hooks.get(hook_name).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Hook not found: {}", hook_name),
        )
    })?;
    // 取得したコマンドの実行
    for command in hook.commands.values() {
        execute_command(&command.run)?;
    }

    Ok(vec![])
}
