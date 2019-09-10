/**
 * 整形
 * 有符号数以i开头，如i32
 * 无符号数以u开头，如u8
 * 位数长度值有：8、16、32、64、arch
 * 其中 arch 长度的有符号类型为isize，无符号类型为usize
 * isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的
 * rust默认数字类型为i32，是最快的
 * 
 * 浮点型
 * rust原生浮点数类型为f32 和 f64 ，分别占32 和 64 位
 * 默认类型是f64
 * 
 * 数值运算
 * rust中所有数字类型都支持基本数学运算：加减乘除和取余
 * 
 * 布尔值
 * true 和 false
 * 
 * 字符类型
 * char 类型是语言中最原生的字母类型，
 * char 由单引号指定， 字符串使用双引号
 * 
 * Rust 的 char 类型代表了一个 Unicode 标量值（Unicode Scalar Value）
 * 这意味着它可以比 ASCII 表示更多内容
 * 在 Rust 中，拼音字母，中文、日文、韩文等字符，emoji以及零长度的空白字符都是有效的 char 值
 * Unicode 标量值包含从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值
*/

fn main() {
    let x: i32 = 123_456_789;
    println!("The value of x is: {}", x);
    
    // rust 中也会存在浮点数计算精度不够的问题
    let y: f64 = 0.1 + 0.2;
    println!("The value of y is: {}", y);

    let z = 43 % 5;
    println!("The value of z is: {}", z);

    let t = true;
    let f: bool = false;
    println!("The value of t is: {} & value of f is: {}", t, f);

    let z: char = 'Z';
    let heart_eye_cat = '😻';
    println!("The value of z is: {} & value of heart_eye_cat is: {}", z, heart_eye_cat);
}
