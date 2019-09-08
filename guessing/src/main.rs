// 引入io库，来自于标准库std
// 默认情况下，std：：prelude的少量内容会被引入到每个程序的作用域中
// 如果需要的类型不在prelude中，则需要使用use语句显式的将其引入作用域
use std::io;

// Rng 是rand库的一个trait，定义了随机数生成器应实现的方法
use rand::Rng;

use std::cmp::Ordering;

fn main() {
    // println-打印宏
    println!("Guess the number!");
    
    // rand::thread_rng 函数提供实际使用的随机数生成器：
    // 它位于当前执行线程的本地环境中，并从操作系统获取 seed
    // 调用随机数生成器的 gen_range 方法,并生成一个指定区间[1,101)的随机数
    let secret_number = rand::thread_rng().gen_range(1,101);

    // println!("The secret number is: {}", secret_number);
    
    // 循环执行
    loop {
        println!("Please input your guess.");

        // let 创建变量
        // rust 中变量默认不可变
        // 变量名前使用 mut 使其可变
        // String::new函数会返回一个标准字符串类型的新实例，事utf-8编码的可增长文本块
        // ::表明new事String类型的关联函数，对应其他语言的“静态方法”概念
        let mut guess = String::new();
        
        // 调用io的关联函数stdin
        // stdin返回一个std::io::Stdin的实例，代表终端标准输入句柄的类型
        // 使用read_line方法从标准输入句柄获取用户输入，将用户键入的任何内容都存入一个字符串中
        // read_line需要传入一个可变字符串作为参数
        // &表示这个参数是一个引用，引用默认不可变，使用&mut的写法使其可变
        // 此处引用了String类型的可变实例guess
        io::stdin().read_line(&mut guess)
        // 用户输入完成后read_line方法返回一个io::Result类型的值
        // Result类型事枚举(enumerations/enums),成员事Ok和Err,分别表示操作成功/失败
        // io::Result实例拥有expect方法
        // 当io::Result的值是Err时,expect会导致程序崩溃,并显示传入的参数作为错误信息
        // 当io::Result的值事是Ok时,expect会获取Ok中的值并原样返回
            .expect("Failed to read line");
        
        // 也可以写成如下形式：
        // std::io::stdin().XXX

        // 创建新的32位无符号类型变量guess
        // rust允许使用一个新值（隐藏shadow）之前的变量值，通常用于数据类型转换的场景
        // parse返回Result类型
        // match处理枚举
        let guess:u32 = match guess.trim().parse() {
          // OK和Err都是Result的枚举
          Ok(num) => num,
          // 当发生错误时继续执行下一轮loop，而不崩溃
          // _是通配符
          Err(_) => continue,
        };
        
        // {}是预留在特定位置的占位符
        // 使用{}可以打印多个值
        // 第n个{}对应格式化字符串后的第n个值
        println!("You guessed: {}",guess);

        // 对比较guess与secret_number的大小
        match guess.cmp(&secret_number){
          // Ordering是一个枚举
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => {
            println!("You win!");
            // 当猜测正确后退出循环
            break;
          }
        }
    }
}
