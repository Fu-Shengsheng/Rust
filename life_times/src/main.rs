fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    {
        let string3 = "long long ago";
        let result = longest(string1.as_str(), string3);
        println!("Longest string is {}", result);
    }

    println!("{}", test());

    let novel = String::from("Call me Ishmael. Some yaers ago…");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}

// 泛型生命周期参数注解，说明两个参数必须拥有相同的生命周期
// 不遵守协议传入的参数会被借用检查器拒绝
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 字符串字面值拥有 'static 生命周期，静态生命周期能够存活于整个程序期间
// 因为字符串文本被直接存储在程序的二进制文件中，二这个文件总是可用的
fn test() -> &'static str {
    let temp_str = "hello world";
    &temp_str
}

// 包含引用的结构体
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 结合泛型类型参数，trait bounds和生命周期
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
