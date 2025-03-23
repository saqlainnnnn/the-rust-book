struct ImportantExcerpt<'a>{
    part: &'a str,
}

fn main() {
    /*let string1 = String::from("hello");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);*/

    let novel = String::from("call me ");

    let string1 = String::from("hello");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("{}", result)
}

fn longest<'a>(x: &'a str, y: & str) -> &'a str {
    x
}