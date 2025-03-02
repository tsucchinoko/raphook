pub fn init_logger() {
    use env_logger::fmt::Color;
    use std::io::Write;

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format(|buf, record| {
            let mut target_style = buf.style();
            let mut args_style = buf.style();

            // ログレベルに応じた色を設定
            match record.level() {
                log::Level::Error => {
                    target_style
                        .set_bg(Color::Red)
                        .set_color(Color::White)
                        .set_bold(true);
                    args_style.set_color(Color::Red).set_bold(true);
                }
                log::Level::Warn => {
                    target_style
                        .set_bg(Color::Yellow)
                        .set_color(Color::Black)
                        .set_bold(true);
                    args_style.set_color(Color::Yellow);
                }
                log::Level::Info => {
                    target_style
                        .set_bg(Color::Green)
                        .set_color(Color::Black)
                        .set_bold(true);
                    args_style.set_color(Color::Green); // 通常のメッセージは白色
                }
                log::Level::Debug => {
                    target_style
                        .set_bg(Color::Blue)
                        .set_color(Color::White)
                        .set_bold(true);
                    args_style.set_color(Color::Blue);
                }
                log::Level::Trace => {
                    target_style
                        .set_bg(Color::Cyan)
                        .set_color(Color::Black)
                        .set_bold(true);
                    args_style.set_color(Color::Cyan);
                }
            };

            // ターゲット名にパディングを追加
            let padded_target = format!(" {} ", record.target());

            writeln!(
                buf,
                "{} {}",
                target_style.value(padded_target),
                args_style.value(record.args())
            )
        })
        .init();
}
