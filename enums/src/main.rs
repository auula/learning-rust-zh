#[derive(Debug)]
enum Colors {
    Red,
    Blue,
    Yellow,
    Green,
}

fn main() {
    let param = Colors::Blue;
    match param {
        Colors::Red => println!("红色"),
        Colors::Blue => println!("蓝色"),
        Colors::Yellow => println!("黄色"),
        Colors::Green => println!("绿色"),
    }
}
