fn main() {
    let age = 22;
    if age > 18 {
        println!("你已经成年了！")
    } else {
        println!("未成年人!")
    }

    let num = 2;
    if num > 0 {
        println!("{} is positive", num);
    } else if num < 0 {
        println!("{} is negative", num);
    } else {
        println!("{} is neither positive nor negative", num);
    }

    //match 语句有返回值，它把 匹配值 后执行的最后一条语句的结果当作返回值

    let month = "二月";

    let english_month = match month {
        "一月" => "January",
        "二月" => "February",
        "三月" => "March",
        "四月" => "April",
        _ => "Unknown",
    };
    println!("{}", english_month)
}
