fn main() {
    let tuples: (&'static str, i8, f64) = ("🦀", 22, 3.1415927);
    println!("{:?}", tuples);
    // 声明一个可变的tuple
    let mut people = ("tom", "robin", "jarvib");
    // 通过下标访问
    println!("{},{},{}", people.0, people.1, people.2);
    // 修改下标为2的值
    people.2 = "Jarvib Ding";
    let (v1, v2, v3) = people;
    println!("{},{},{}", v1, v2, v3);
}
