// Rust 标量数据类型
// 标量数类型: 只有一个值，复合类型是通过标量类型构成的

// 1. 整数类型
// 2. 浮点类型
// 3. 布尔类型
// 4. 字符类型
fn main(){
    // 整型
    let i = 00_32; // default scalar type i32
    let age:u8 = 0x16;
    let num:i64 = -00_64;
    let iarch:isize = i; 
    let uarch:usize = 64;
    println!("i = {} age = {} num = {}",i,age,num);
    println!("iarch = {} uarch = {}",iarch,uarch);

    // 浮点型
    let f = 64.00;
    println!("f is {:?}",f);
    // let f:i64 = f;
    // println!("f is {:?}",f)

    // 布尔类型
    let b = true;
    println!("b = {:?}",b);

    // 字符
    let special_character = '@'; //default
    let alphabet:char = 'A';
    let chinese:char = '丁';
    let emoji:char = '😜'; // 笑脸的那个图

    println!("special character is {}",special_character);
    println!("alphabet is {}",alphabet);
    println!("chinese is {:?}",chinese);
    println!("emoji is {}",emoji);
}