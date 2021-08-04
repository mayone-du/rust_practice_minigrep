use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// コマンドライン引数
pub struct Config {
  pub query: String,
  pub filename: String,
}

// new関連関数を追加
impl Config {
  // 引数で標準入力の引数を受け取り、Result型を返す
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    // 標準入力の引数の数をチェック
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    // 最初の引数を検索する文字列とする
    let query = args[1].clone();
    // 2個目の引数にファイル名を受け取る
    let filename = args[2].clone();
    // 問題なければResultのOKを返す
    Ok(Config { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
  // ファイルを開き、エラーだったらResultでエラーを返す
  let mut f = File::open(config.filename)?;

  // 可変文字列を定義
  let mut contents = String::new();
  // ファイルの内容を↑の可変文字列にバインド
  f.read_to_string(&mut contents)?;

  println!("With text:\n{}", contents);

  // うまく行ったらResultのOkを返す
  Ok(())
}
