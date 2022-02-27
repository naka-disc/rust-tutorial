// 標準ライブラリ 入力受付など
use std::io;
// 標準ライブラリ 比較
// OrderingはEnumで、Less/Greater/Equalを持っており、これらは小さい、同値、大きいを表す
use std::cmp::Ordering;
// randライブラリ
use rand::Rng;

// main関数がエントリーポイント
fn main() {
  // println!で、画面上に出力
  println!("Guess the number!");

  // 1〜100までのランダムな数値を定義
  let secret_number = rand::thread_rng().gen_range(1..101);
  println!("The secret number is: {}", secret_number);

  // 正解とマッチするまでループ
  loop {
    // 入力を促す出力
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

    // このままだと型が不一致なので、入力された値をパース
    // u32は、符号なし32ビットの数値
    let guess: u32 = guess
      // 先頭と末尾のスペースを除去
      .trim()
      // パース。値に応じて、型推論してくれる
      // @see https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse
      .parse()
      // パニック処理
      // あんまりよくわからんここ
      .expect("Please type a number!");

    // 乱数と変換後の入力値とを比較し、結果を出力
    match guess.cmp(&secret_number) {
      // guess < secret_number
      Ordering::Less => println!("Too small!"),
      // guess > secret_number
      Ordering::Greater => println!("Too big!"),
      // guess = secret_number
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }

}
