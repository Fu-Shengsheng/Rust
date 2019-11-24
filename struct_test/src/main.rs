// derive注解，以便可以以debug形式打印出struct内容
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 为结构体定义方法
// 方法的第一个参数总是self，并代表该方法的结构体实例
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width1 = 100;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixcels",
        area(width1, height1)
    );

    let rect1 = (80, 40);
    println!("The area of the rect1 is {} square pixcels", area1(rect1));

    let rect2 = Rectangle {
        width: 90,
        height: 45,
    };
    // 以debug模式打印struct内容，此时一行打印出所有内容
    println!("rect2 is {:?}", rect2);
    // 以debug模式打印struct内容，并且以更易读的形式
    println!("rect2 is {:#?}", rect2);
    println!("The area of the rect2 is {} square pixcels", area2(&rect2));

    let rect3 = Rectangle {
        width: 50,
        height: 120,
    };
    println!("rect3 area is {}", rect3.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
