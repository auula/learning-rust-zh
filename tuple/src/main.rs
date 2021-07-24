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

    // 二维的元组
    let tuples_2d = (1024, ("Rust", '🦀'));

    println!("tuples_2d.1.1 = {}", tuples_2d.1 .1);

    // 指定数据类型
    let tup_type: (i8, i32, bool) = (21, -1024, true);
    // 解构元素
    let (one, two, three) = tup_type;
    // 二维的元组
    let tup_2d: (f64, (i8, i32, bool)) = (3.1415927, (one, two, three));
    println!("tup_2d = {:?}", tup_2d);
    // 索引
    println!("π = {:?}", tup_2d.0);
}
