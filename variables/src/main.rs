fn main() {
    // 使用 mut 关键字声明可变变量
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // x = "hello";
    // println!("The value of x is: {}", x);

    // rust允许数字字面值中插入下划线提升可读性
    const MAX_POINTS: u32 = 100_000;

    println!("The const variable is {}", MAX_POINTS);

    // 变量隐藏
    // 每次实际上创建了一个新的默认不可变变量
    let y = 1;

    let y = y + 2;

    let y = y * 3;

    println!("The value of y is {}", y);
}
