mod front_of_the_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_the_house {
    pub struct Breakfast{
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
                Breakfast{
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
        }
    }
}

pub fn eat_at_restaurant() {
   let mut meal = back_of_the_house::Breakfast::summer("rye");

   let meal2 = back_of_the_house::Breakfast {
        toast: String::from("wheat"),
        seasonal_fruit: String::from("peaches")
   }
}

fn serve_order() {}