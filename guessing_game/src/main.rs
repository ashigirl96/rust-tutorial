extern crate rand;

// 標準入出力ライブラリ
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);


    // Rustは値`Result`を使わないと警告されるので, .expectできちんとエラー処理する必要がある
    loop {
        println!("Please input your guess");

        // letはguessという名前の束縛を作り, String::new()という値に束縛.(要は変数)
        // mutは可変にする
        // newはStringのstatic methodであり, String自体に関連付けられる(インスタンスではない)
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse()
        //    .expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}