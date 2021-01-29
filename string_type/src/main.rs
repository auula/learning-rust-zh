// Rust中的字符串

// 1. Rust中的字符串面量
// 2. String标准库

const SHANG_HAI: &str = "上海";

fn main() {
    // 常量只能 被赋值为 常量表达式/数学表达式，不能是 函数返回值 或者其它只有在运行时才能确定的值。
    println!("SHANG_HAI = {}", SHANG_HAI);

    // 字符串字面量模式是 静态的 这就意味着字符串字面量从创建时开始会一直保存到程序结束
    let city = "上海";
    let programmer = "rust coder";
    println!("city {0} programmer = {1}", city, programmer);
    // 字符串面量默认就是“静态”的 你可以通过static关键字显示声明
    let name: &'static str = "Jarvib Ding";
    println!("my name is {name}", name = name);

    // 字符串对象
    let mut phone = String::new();
    phone.push_str("Apple iPhone 11");
    let iphone = String::from(phone);
    println!("{}", iphone);

    // 1 {{Hello}}
    let mut hello_string = String::new();
    hello_string.push_str("{");
    hello_string.push_str("Hello");
    hello_string.push_str("}");
    println!("{}", hello_string);
    // 2 {{Hello}}
    println!("{left}Hello{right}", left = "{", right = "}");
    // 3 {{Hello}}
    println!("{{{}}}", "hello");
}
