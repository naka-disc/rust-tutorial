// 標準ライブラリ 入力受付など
use std::io;

// main関数がエントリーポイント
fn main() {
  // println!で、画面上に出力
  println!("Guess the number!");
  println!("Please input your guess.");

  // 変数定義 let mutなので、ミュータブル（変更可能）
  let mut guess = String::new();

  // read_lineは、ユーザーからの入力を受け付ける標準ライブラリの関数
  io::stdin()
    // 上で定義したguess変数に入力値を入れる
    // &mut guess は、引数が3章であることを示し、mutに&をつけるのは、Rustがデフォルトでイミュータブルだから？
    .read_line(&mut guess)
    // パニック処理
    // あんまりよくわからんここ
    .expect("Failed to read line");

  // 入力値をセットして出力
  println!("You guessed: {}", guess);
}
