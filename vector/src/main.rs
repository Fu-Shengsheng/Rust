fn main() {
    // vector 是泛类型的，所以声明空的vector时要带上类型注解
    let v: Vec<i32> = Vec::new();

    // 也可以使用初始值来创建vector，此时rust会自动推断类型
    // 新建有值的vec!宏
    let v1 = vec![1, 2, 3];

    // 使用push对vector增加元素
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);

    // 读取vector的值
    // 1.索引语法
    let second: &i32 = &v1[1];
    // 2.get方法
    match v1.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third elemnt"),
    }
    println!("second {}", second);

    // 遍历
    for i in &v1 {
        println!("{}", i);
    }

    for i in &mut v2 {
        // 解引用运算符 * 可以获取到引用中的值
        *i += 50;
        println!("{}", i);
    }

    // 使用枚举存储多种类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    };
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hahah")),
        SpreadsheetCell::Float(10.12),
    ];
}
