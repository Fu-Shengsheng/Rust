// 控制流

fn main() {
    let number = 3;

    // if表达式用于条件检查
    // if表达式中与条件相关联的代码块也被叫做arms
    // 条件必须是bool值
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    let mut x = 8;
    if_test(x);
    x = 6;
    if_test(x);
    x = 2;
    if_test(x);

    let y = let_if_test(false);
    println!("The value of y is: {}", y);

    let z: i32 = 4;
    let result_loop = loop_test(z);
    println!("The value of result_loop is: {}", result_loop);

    let q: i32 = while_test(0);
    println!("The value of q is: {}", q);

    let arr: [i32; 5] = [10, 8, 6, 4, 2];
    for_test(arr);

    for_test_2();

    fibonacci_sequence(10);
}

fn if_test(i: i32) {
    if i % 4 == 0 {
        println!("number is divisible by 4");
    } else if i % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }
}

fn let_if_test(conditor: bool) -> i32 {
    // 可以使用if表达式的结果给变量赋值
    // 代码块的值是其最后一个表达式的值
    // 此时if语句的每个分支的可能的返回值都必须是相同类型
    let number: i32 = if conditor { 9 } else { 6 };

    // 数字本身就是表达式
    number
}

fn loop_test(mut param: i32) -> i32 {
    // break表达式可以停止循环
    // 如果循环后仍需要某个值，可以将该值放在break表达式后，以便在循环外使用
    let result = loop {
        param += 1;

        if param == 10 {
            break param * 2;
        }
    };

    result + param
}

fn while_test(mut param: i32) -> i32 {
    // 当条件为真时，使用while循环运行代码
    while param != 100 {
        println!("{}!", param);
        param += 1;
    }

    param / 2
}

fn for_test(arr: [i32; 5]) {
    // for循环具备简洁性和安全性，时rust中使用最多的循环结构
    for element in arr.iter() {
        println!("The value is: {}", element / 2);
    }
}

fn for_test_2() {
    // rev用来反转range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn fibonacci_sequence(order: i32) {
    // let mut arr =
    let mut result: i32 = 0;
    let mut last_result: i32 = 0;
    let mut last_last_result: i32 = 0;
    for number in (0..order) {
        if result == 0 {
            result = 1;
        } else {
            last_last_result = last_result;
            last_result = result;
            result = last_result + last_last_result;
        }
        println!("{}!", result);
    }
}
