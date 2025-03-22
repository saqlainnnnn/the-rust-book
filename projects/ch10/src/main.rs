

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];


    println!("the largest number is {}", get_largest(number_list));
}

fn get_largest(number_list : Vec<i32>) ->i32 {
    
    let mut largest = number_list[0];

    for number in number_list{
        if number > largest{
            largest = number;
        }
    }

    return largest;
}
