

//OWNERSHIP RULES
/*
    1. EACH VALUE IN RUST HAS A VARIABLE THATS CALLED ITS OWNER
    2. THERE CAN ONLY BE ONE OWNER AT A TIME
    3. WHEN THE OWNER GOES OUT OF SCOPE THE VALUE WILL BE DROPPED.
*/

use std::string;

fn main() {
    println!("Hello, world!");
    
    { 
        let s = "hello";
    } // S SCOPE IS OVER NOW 

    {
        let mut s = String::from("hello"); // data allocated to heap, good for user-inputs and stuff like that
        
        s.push_str(", world"); 

        println!("{s}");

    }

    {
        let s1 = String::from("hello");
        let s2 = s1; // value of s1 moved to s2 ie s1 is out of scope and we cant use it, its not a shallow copy
                             // the value hello is saved on heap and ptr, len and capacity are stored on stack when we do s2= s1 values 
                             // on stack are copied instead of the whole heap 

    }

    let s = String::from("hello");
    
    takes_ownership(s);
    
    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_int: i32) {
    println!("{some_int}");
}

fn gives_ownership() ->String {

    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} 

