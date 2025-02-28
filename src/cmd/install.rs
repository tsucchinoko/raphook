use crate::raphook;
use std::fs::{self, File};
use std::io::{self, Read, Write};

use std::path::Path;

pub const AVAILABLE_HOOKS: &[&str] = &[
    "pre-commit",
    "prepare-commit-msg",
    "commit-msg",
    "post-commit",
    "pre-push",
];

fn ensure_config_file_exists(path: &str) -> io::Result<String> {
    let config_file = Path::new(path).join("raphook.yml");

    if !config_file.exists() {
        // TODO: このファイルは配信されないため、インストール時にエラーが発生する
        let template_path = Path::new(path)
            .join("src")
            .join("template")
            .join("config-template.yml");

        let mut template = String::new();
        File::open(&template_path)?.read_to_string(&mut template)?;

        // テンプレートファイルの書き込み
        let mut file = File::create(&config_file)?;
        file.write_all(template.as_bytes())?;
    }
    Ok(config_file.to_string_lossy().into_owned())
}

fn ensure_hooks_dir_exists(path: &str) -> io::Result<String> {
    let hooks_dir = Path::new(path).join(".git").join("hooks");
    if !hooks_dir.exists() {
        fs::create_dir_all(&hooks_dir)?;
    }
    Ok(hooks_dir.to_string_lossy().into_owned())
}

fn install_hook(hooks_dir: &str, hook_name: &str) -> std::io::Result<()> {
    let hook_path = Path::new(hooks_dir).join(hook_name);
    let template_path = Path::new("src/template/hook-template.sh");

    // テンプレートファイルを読み込む
    let mut template = String::new();
    File::open(template_path)?.read_to_string(&mut template)?;

    // {hook_name}をフック名に置き換え
    let hook_script = template.replace("{hook_name}", hook_name);

    // ファイルに書き込み
    let mut file = File::create(&hook_path)?;
    file.write_all(hook_script.as_bytes())?;

    // 実行権限を付与（Unixシステムの場合）
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook_path)?.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(&hook_path, perms)?;
    }

    Ok(())
}

pub fn install(path: &str) -> io::Result<Vec<String>> {
    // 設定ファイルの存在確認
    let _ = ensure_config_file_exists(path)?;

    // 設定ファイルの読み込み
    let config = raphook::config::Config::load(path)?;

    // Git hooksの存在確認
    let hooks_dir = ensure_hooks_dir_exists(path)?;

    // 利用可能なフックか確認
    let mut valid_hooks = Vec::new();
    for hook in config.hook_names().iter() {
        if !AVAILABLE_HOOKS.contains(&hook) {
            eprintln!("Invalid hook: {}", hook);
            continue;
        }
        valid_hooks.push(hook.to_string());
    }

    // Git hooksのインストール
    let mut installed_hooks = Vec::new();
    for hook in valid_hooks.iter() {
        match install_hook(&hooks_dir, hook) {
            Ok(_) => installed_hooks.push(hook.to_string()),
            Err(e) => eprintln!("Failed to install {}: {}", hook, e),
        }
    }

    Ok(installed_hooks)
}
