// Rust中的变量
// Author: SDing <deen.job@qq.com>

// 1. 不可变变量
// 2. 可变变量
// 3. 变量的重影

fn main(){
    // 通过let 声明的变量是不可变的
    let v = "variable";
    println!("这是一个不可变的变量: {}",v);
    // unexpected token
    //v = "var 2" 
    //println!("{}",v)
    let v = "new ".to_owned() + v;
    println!("重影: {}",v);

    // 通过mut关键字声明的变量可以被修改
    let mut m = 123;
    //println!("m assigned value is {}",m);
    m = 666 + m;
    println!("这是一个可变变量: {}",m);

    // 重影 Shadowing
    let s = 32;
    let s = 32 + s;
    println!("s value is {}",s);
    let s = s - 32;
    println!("s value assigned value is {}",s);
    // 运行代码编译器没有报错，很多人就奇怪了，let声明的变量不是不可变吗？
    // 对的，let关键字重新声明的相同变量的名的变量会砍掉之前的变量
    // 并且如果需要之前变量的值就会拿到值然后在删除掉，重新分配

    // 可以通过查看内存地址就查看
    println!("old s pointer is {:p}",&s);
    let s = s;
    println!("new s pointer is {:p}",&s);

    // 补充常量
    // 定义常量时必须指定数据类型，而定义变量时数据类型可以省略
    const MY_AGE:u64 = 21;
    println!("my age is {}",MY_AGE)
} 