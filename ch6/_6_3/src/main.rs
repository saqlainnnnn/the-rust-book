#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("the max is configured to be {max}")
    }

    let mut count = 0; 
    if let Coin::Quarter(state) = count {
        println!("State quarter from {state:?}");
    } else {
        count +=1;
    }
}
