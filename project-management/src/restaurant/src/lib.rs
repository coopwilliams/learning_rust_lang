mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// specifying the parent directory is idiomatic (unlike in Python)
pub use crate::front_of_house::hosting;
use crate::back_of_house::Breakfast;

pub fn eat_at_restaurant() {

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // changed my mind
    // can't modify the seasonal fruit that comes with the breakfast
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}