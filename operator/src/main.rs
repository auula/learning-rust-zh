fn main() {
    // 加减乘除
    let result = 11 + 11;
    println!("11 + 11 = {}", result);
    println!("11 - 11 = {}", 11 - 11);
    println!("11 * 11 = {}", 11 * 11);
    println!("11 / 11 = {}", 11 / 11);
    println!("11 % 11 = {}", 11 % 11);

    // 关系运算
    println!("1 > 0 {}", 1 > 0);
    println!("0 < 1 {}", 0 < 1);
    println!("21 >= 21 {}", 21 >= 21);
    println!("21 <= 21 {}", 21 <= 21);
    println!("0 == 0 {}", 0 == 0);
    println!("0 != 0 {}", 0 != 0);

    // 逻辑与  逻辑或 逻辑非 (取反)
    println!("1 == 1 && 0 != 1 {}", 1 == 1 && 0 != 1);
    println!("1 == 1 || 0 != 1 {}", 1 == 1 || 0 != 1);
    println!("!(1 == 1) {}", !(1 == 1));

    let a = 20;
    let b = 30;

    if (a > 10) && (b > 10) {
        println!("true");
    }
    let c = 0;
    let d = 30;

    if (c > 10) || (d > 10) {
        println!("true");
    }
    let is_elder = false;

    if !is_elder {
        println!("Not Elder");
    }
}
