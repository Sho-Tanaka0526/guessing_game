use std::io;                                //入出力のioライブラリの取り込み

fn main() {
    println!("Guess the number!");          //数を当ててごらん

    println!("Please input your guess.");   //ほら、予想を入力してね

    let mut guess = String::new();          //ユーザの入力を格納する変数

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");     //行の読み込みに失敗しました

    println!("You gessed: {}", guess);  //次のように予想しました: {}
    
}