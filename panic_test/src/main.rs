use std::fs::File;
use std::io::ErrorKind;

use std::io;
use std::io::Read;

fn main() {
    // open返回Result<T,E>类型的值，T放入成功值类型，E放入失败值类型
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // 使用match进一步处理失败场景
        Err(error) => match error.kind() {
            // 如果错误类型是NotFound则创建对应的文件
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file :{:?}", e),
            },
            other_error => panic!("Problem opening the file :{:?}"),
        },
    };

    // 使用expect来处理Result<T,E>
    let f_1 = File::open("hello1.txt").expect("Failed to open hello1.txt");

    read_username_from_file();
}

// 使用?简写处理错误
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("../hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
