/**
 * 整形
 * 有符号数以i开头，如i32
 * 无符号数以u开头，如u8
 * 位数长度值有：8、16、32、64、arch
 * 其中 arch 长度的有符号类型为isize，无符号类型为usize
 * isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的
 * rust默认数字类型为i32，是最快的
*/

fn main() {
    let x: i32 = 123_456_789;
    println!("The value of x is: {}", x);
}
