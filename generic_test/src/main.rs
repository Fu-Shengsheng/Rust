fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.5, y: 3.7 };
    let model = Model { x: 1, y: 5.3 };

    let p1 = Point1 { x: 5, y: 10.4 };
    let p2 = Point1 { x: "hello", y: "c" };
    let p = p1.mixup(p2);

    println!("p.x={},p.y={}", p.x, p.y);

    let number_list_1 = vec![3, 5, 1, 2, 4];
    let result_1 = largest_i32(&number_list_1);

    let number_list_2 = vec![16, 12, 13, 17, 15, 14];
    let result_2 = largest(&number_list_2);

    let char_list = vec!['h', 'y', 'c', 'v', 'i'];
    let result_3 = largest(&char_list);

    println!("largest number is {} & {}", result_1, result_2);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

// 结构体中使用泛型
struct Point<T> {
    x: T,
    y: T,
}
// 泛型方法定义
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}
// 定义f32类型的Point实例独有方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point1<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point1<T, U> {
    fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

// 多个不同类型值的泛型结构体
struct Model<T, U> {
    x: T,
    y: U,
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 使用泛型抽象公共函数，可以同时处理不同数据类型的集合
// trait Bounds中指定了传入的参数必须实现PartialOrd（用于比较）和Copy(用于在栈上进行数据拷贝)两个trait
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 定义trait
pub trait Summary {
    // 默认实现
    fn summarize(&self) -> String {
        String::from("Read more...")
    }

    fn summarize_author(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 使用默认的 summarize 实现
impl Summary for NewArticle {
    // 没有默认实现的需要在引入时进行实现
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // 重载 summarize
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// trait作为参数, 可以传入实现了Summary的实例（如Tweet和NewsArticle）
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// trait Bound
pub fn notify_T<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
