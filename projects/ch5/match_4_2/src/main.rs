#[derive(Debug)]

enum UsState {
    Albama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
}

fn value_in_cents(coin : Coin) -> u8 {
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
