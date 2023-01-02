mod front_of_house;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

//    meal.seasonal_fruit = String::from("blueberries");
}

fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {
	Soup,
	Salad,
    }
    
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
	super::serve_order();
    }

    fn cook_order() {}
}
