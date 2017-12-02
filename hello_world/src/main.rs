fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    // !が関数の後ろについてたら, それは関数はなく, マクロ
    println!("Hello, world!");

    println!("10 + 1 => {}", add_one(10));
}
