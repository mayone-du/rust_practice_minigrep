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

  // 検索し出力する
  for line in search(&config.query, &contents) {
    println!("{}", line);
  }
  // うまく行ったらResultのOkを返す
  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    // Rustは
    // 安全で速く生産性も高い。
    // 3つ選んで。
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
}

// ライフタイムを指定
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  // 返り値用のベクター型変数
  let mut results = Vec::new();

  // ファイルの内容を1行ずつforループで回し、各行内に検索したい文字列が含まれているかを確認
  for line in contents.lines() {
    // 含まれていたら、その行をベクターにpush
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}
