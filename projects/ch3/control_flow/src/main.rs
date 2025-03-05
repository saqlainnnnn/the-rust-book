fn main() {
    let condition = true;
    let number = if condition {5} else {6};
    
    println!("{number}");


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("loop {result}");
}
