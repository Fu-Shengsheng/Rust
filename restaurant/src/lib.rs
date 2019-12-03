// 定义模块，整个模块都植根于名为crate的隐式模块下
mod front_of_house {
    // 定义内嵌模块
    // 使用Pub关键字标记模块，使其可以被外部访问
    pub mod hosting {
        // 公有模块内部的项不一定公有
        pub fn add_to_waitlist() {
            println!("add to waitlist");
        }
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}

// rust中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的
// 父模块中的项不能使用子模块中的私有项
// 子模块中的项可以使用父模块中的私有项
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn server_order() {
    println!("server order");
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 使用super起始的相对路径可以从父模块访问
        super::server_order();
    }

    fn cook_order() {}

    // 创建公有结构体
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

// 结构体公有，内部项默认私有
// 枚举公有，内部项默认公有
