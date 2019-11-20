// 所有权（ownership）是rust的核心功能之一
// 它使得无需进行gc即可保证内存安全

// stack中的所有数据都必须占用已知且固定的大小
// 在编译时大小未知或可能发生变化的数据需要存放在heap（堆）中
// 访问堆上的数据比栈慢

// 调用函数时传递给函数的值和函数的局部变量被压入栈中，函数结束时被移除

// 跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的。

fn main() {
    println!("Hello, world!");

    let mut s = String::from("wowowo hahaha");
    let word = first_word(&s);
    println!("index is {}", word);
    // s.clear();
    // println!("index is {}", word);

    let s1 = s.clone();

    // 字面值就是slice
    // slice_word其实是字面值类型
    let mut slice_word = first_word_slice(&s1);
    println!("slice_word is {}", slice_word);

    // 字面值就是slice
    // slice_temp其实是字面值类型
    let slice_temp = &slice_word[..2];
    slice_word = "aaaaaa";
    println!("{}", slice_temp);
    println!("slice_word is {}", slice_word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        println!("{}  {}  {}", i, item, b' ');
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// 字符串slice的类型声明是&str
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
