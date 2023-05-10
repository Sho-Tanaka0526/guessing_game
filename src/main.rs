use std::io;                                //入出力のioライブラリの取り込み
use rand::Rng;                              //乱数生成クレートを導入する
use std::cmp::Ordering;                     //比較関数？

fn main() {
    println!("Guess the number!");          //画面に表示する

    let secret_number = rand::thread_rng().gen_range(1..101);   //1から100の乱数を生成する

    //println!("The secret number is: {}", secret_number);    //秘密の数字は次の通り: {}  テスト用に表示する

    println!("Please input your guess.");   //数字を入力してください

    let mut guess = String::new();          //ユーザの入力を格納する変数

    io::stdin()                             //ユーザの入力を受け取る
        .read_line(&mut guess)              //可変変数guessを使う
        .expect("Failed to read line");     //行の読み込みに失敗しました

    println!("You gessed: {}", guess);      //次のように予想しました: {}
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),   //小さすぎ！
        Ordering::Greater => println!("Too big!"),  //大きすぎ！
        Ordering::Equal => println!("You win!"),    //やったね！
    }
}