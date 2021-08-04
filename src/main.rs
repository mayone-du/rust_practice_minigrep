extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;
fn main() {
    // 標準入力で受け取った引数（コマンドライン引数）をベクター型で保持
    let args: Vec<String> = env::args().collect();

    // new関連関数からResult型を受け取りエラーの場合はプロセスを終了
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("引数解析時にエラーが発生しました。: {}", err);
        process::exit(1);
    });
    println!(
        "{}の中から\"{}\"を探しています...",
        config.filename, config.query
    );
    if let Err(e) = minigrep::run(config) {
        println!("アプリケーションエラー: {}", e);
        process::exit(1);
    }
}
