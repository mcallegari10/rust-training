mod front_of_house;

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
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
        Salad
    }

    fn fix_incorrect_order() {
        super::serve_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;
// self brings the modules at the same level as "relative" path
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // use add_to_waitlist with the use keyword to import moduules
    hosting::add_to_waitlist();
}
