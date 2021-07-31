type MathOp = fn(i32, i32);

fn add(x: i32, y: i32) {
    println!("{} - {} = {}", x, y, x + y)
}

fn sub(x: i32, y: i32) {
    println!("{} - {} = {}", x, y, x - y)
}

fn MathFunc(op: MathOp, tup: (i32, i32)) {
    op(tup.0, tup.1);
}

fn main() {
    // 高阶函数
    MathFunc(add, (100, 200))
}
