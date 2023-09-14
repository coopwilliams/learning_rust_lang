pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house {
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

fn deliver_order() {}

mod back_of_house {
    // making an enum public
    // makes all of its variants public.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // making a struct public does NOT
    // make all its fields public.
    // Here, seasonal_fruit is private.
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

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    // hosting is a valid name in scope
    // because of "use" of 'hosting' in
    // the same scope.
    hosting::add_to_waitlist(); 

    // order a summer breakfast with rye
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    // // can't update the fruit, cuz it's private
    // meal.seasonal_fruit = String::from("pears");

    // // also can't create a Breakfast directly
    // // because it has private fields.
    // // Let the chef do that!
    // let diy_meal = back_of_house::Breakfast {
    //     toast: String::from("Rye"), 
    //     seasonal_fruit: String::from("peaches")
    // };

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}