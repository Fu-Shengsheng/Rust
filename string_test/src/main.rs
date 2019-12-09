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
}
