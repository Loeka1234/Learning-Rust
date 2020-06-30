// You can use "use" with the as keyword:
// use std::io::Result as IoResult;

/*
use std::cmp::Ordering;
use std::io;

Better usage:
use std::{cmp::Ordering, io}

Other example:
use std::io;
use std::io::Write;
---> use std::io::{self, Write};

If we want to bring in all items, use the glob operator:
use std::collections::*;
*/

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting;

fn main() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    print!("{:?}", meal);

    meal.toast = String::from("Wheat");
    println!("{:?}", meal);

    // Next line won't compile because seasonal_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    // We can use these bcs enum Appetizer is public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // We can use this bcs we added the module hosting to the scope
    hosting::add_to_waitlist();
}
