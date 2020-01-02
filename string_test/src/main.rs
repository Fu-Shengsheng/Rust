// Rust的核心语言只有一种字符串类型：str，字符串slice，存储在二进制文件的特定位置
// 字符串slice是一些存储在别处的utf-8编码字符串数据的引用

// 称作String的类型由标准库提供，未写入核心语言部分
// String是可增长的、可变的、有所有权的、UTF-8编码的字符串类型

fn main() {
    let tempstr: &str = "Hello";
    println!("{}, world!", tempstr);

    let mut s = String::new();
    s = String::from("first");

    let data = "init contents";
    // 使用to_string方法从字符串字面值创建String
    let s1 = data.to_string();

    let s2 = String::from(data);
    // 使用push_str向String附加字符串slice
    s.push_str(&s2);

    println!("s is {}", s);

    let s3 = String::from("hello, ");
    let s4 = String::from("Jack");
    // 获取 s3 的所有权，附加上从 s4 中拷贝的内容，并返回结果的所有权
    let s5 = s3 + &s4;
    println!("s5 is {}", s5);

    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");
    // format!返回一个带有结果内容的String，并且不会获取任何参数的所有权
    let tempString = format!("{}-{}-{}", s6, s7, s8);
    println!("tempString is {} ", tempString);
    let s9 = s6 + "-" + &s7 + "-" + &s8;
    println!("s9 is {} ", s9);

    // rust 使用utf-8编码存储String，禁止使用索引访问字符串特定位置的字符
    // 需要使用字符串slice的形式获取String部分内容
    let cong = "yes you win";
    let sub = &cong[1..2];
    println!("sub is {}", sub);
    println!("cong length {}", cong.len());

    // 遍历
    // chars方法会将String的每个字符分开并返回
    for c in cong.chars() {
        println!("{}", c);
    }
    // bytes方法返回每一个原始字节
    for c in cong.bytes() {
        println!("{}", c);
    }
}
