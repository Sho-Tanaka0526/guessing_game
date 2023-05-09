use std::io;                                //入出力のioライブラリの取り込み

fn main() {
    println!("Guess the number!");          //画面に表示する

    println!("Please input your guess.");   //数字を入力してください

    let mut guess = String::new();          //ユーザの入力を格納する変数

    io::stdin()                             //ユーザの入力を受け取る
        .read_line(&mut guess)              //可変変数guessを使う
        .expect("Failed to read line");     //行の読み込みに失敗しました

    println!("You gessed: {}", guess);      //次のように予想しました: {}
    
}