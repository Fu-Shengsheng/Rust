/**
 * fn 关键字用于声明新函数
 * 函数和变量名使用snake case风格
 *
 * 函数可以传参
 * 函数签名中必须声明每个参数的类型
 * 参数间使用逗号分隔
 */

fn main() {
    println!("Hello, world!");

    another_function(99_999);

    some_func('A', 666);
}

fn another_function(value: i32) {
    println!("Another function & the value of argument is :{}", value);
}

fn some_func(key: char, value: i32) {
    println!("Your key is :{} & value is :{}", key, value);
}
