// 在rust中通常一在一个数到另外一个数的整数序列中的所有数字我们叫范围类型
fn main() {
    print!("(1..5)=");
    for i in 1..5 {
        print!("{}", i);
    }
    println!();
    print!("(1..=5)");
    for i in 1..=5 {
        print!("{}", i);
    }
    println!();
    // 1-5累加求和
    let sum: i64 = (1..=5).sum();
    println!("sum = {}", sum);
    // 生成1-5的数 然后反转
    let rev = (1..=5).rev();
    for i in rev {
        print!("{}", i)
    }
    println!();
}
