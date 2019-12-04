// 文件名即父模块名
// 定义内嵌模块
// 使用Pub关键字标记模块，使其可以被外部访问
pub mod hosting;
mod serving {
    fn take_order() {}
    fn server_order() {}
    fn take_payment() {}
}
