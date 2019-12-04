// 声明在领域个与模块同名的文件中加载模块的内容
mod front_of_house;

// 使用use关键字将名称引入作用域
pub use crate::front_of_house::hosting;

// rust中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的
// 父模块中的项不能使用子模块中的私有项
// 子模块中的项可以使用父模块中的私有项
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
    // use一般引入目标模块的父模块
    hosting::add_to_waitlist();

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

// 使用as关键字提供新的名称
// 可以避免引入模块名称重复
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}
fn function2() -> IoResult {}

// 使用嵌套路径消除大量的use行
// 一个use语句同时引入多个带有相同前缀的项
use std::{cmp::Ordering, io};
// 使用self将父同子模块一同引入
use std::io::{self, Write};

// 通过global运算符将所有的公有定义引入作用域
use std::collections::*;
