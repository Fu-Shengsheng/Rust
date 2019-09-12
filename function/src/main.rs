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
    
    // rust 的语句并不返回值，有别于C
    // 所以不能使用 let x = y = 1 的写法
    let x= 6;

    // rust 的表达式返回值
    // 函数是一个表达式
    // 宏调用是一个表达式
    // 创建新作用域的大括号（{}代码块）也是一个表达式
}

fn another_function(value: i32) {
    println!("Another function & the value of argument is :{}", value);
}

fn some_func(key: char, value: i32) {
    println!("Your key is :{} & value is :{}", key, value);
}
