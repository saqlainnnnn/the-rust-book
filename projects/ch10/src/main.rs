

// struct Point<T, U> {
//     x : T,
//     y : U,
// }

// struct Point1<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point1<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     println!("the largest number is {}", get_largest(number_list));

//     let one_struct = Point { x: 5 , y : 6.2};
//     let two_struct = Point { x: 5.0 , y : 6};
// }

// fn get_largest<T: PartialOrd + Copy>(number_list : Vec<T>) -> T {
    
//     let mut largest = number_list[0];

//     for number in number_list{
//         if number > largest{
//             largest = number;
//         }
//     }

//     return largest;
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T,U> {
    fn mixup<W, V> (self, other: Point<W, V>) -> Point<T, V> {
        Point { x: self.x, y: other.y }
    }
}

fn main() {
    let p1 = Point {x: 5, y: 3.0};
    let p2 = Point {x: 5, y: "char"};
}