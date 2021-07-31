fn main() {
    let x: i32 = 960;
    println!("{:?}", x);
    let x: i64 = x as i64;
    println!("{}", x);
    let x: char = '丁';
    println!("{}", x);
    let x: u8 = x as u8;
    println!("{}", x);
    let strs = "21".to_string();
    let num = strs.parse::<i8>().unwrap();
    println!("num = {}", num)
}
