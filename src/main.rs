use std::io;                                //入出力のioライブラリの取り込み

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();          //ユーザの入力を格納する変数

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");     //行の読み込みに失敗しました

    println!("You gessed: {}", guess);      //次のように予想しました: {}
    
}