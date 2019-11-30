enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 为枚举定义方法
impl Message {
    fn call(&self) {
        // ...
        // println!("{}", self);
    }
}

// option类型的枚举，是rust系统提供的一种广应用的类型，无需引入，直接使用
// <T>语法是一个泛型类型参数，即Some成员可以包含任意类型的数据
// enum Option<T> {
//     Some(T),
//     None,
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 7,
        Coin::Quarter(state) => {
            println!("State quater from {:?}!", state);
            9
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    // 使用None值时需要显示声明具体的option类型
    let absent_number: Option<i32> = Option::None;

    // option<T>和T时不同类型，不能直接进行混合计算
    // 需要将option<T>转换为<T>

    let result = value_in_cents(Coin::Penny);
    println!("coin penny value:{}", result);

    let result1 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("coin quarter value:{}", result1);

    let some_five = Some(5);
    let six = plus_one(some_five);
    let none = plus_one(None);
    // println!("{},{},{}", some_five, six, none);

    // 使用 _ 通配符进行剩余所有值的匹配
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => println!("other"),
    }

    let coins = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coins {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count {}", count);
}
