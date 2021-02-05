fn main() {
    let mut sum: u16 = 0;
    for i in 1..100 {
        sum += i;
    }
    println!("sum = {}", sum);

    sum = 0;
    while sum < 100 {
        sum += 1;
    }
    println!("sum = {}", sum);

    let mut n = 0;
    loop {
        if n == 100 {
            // stop action
            break;
        }
        n += 1;
    }
    println!("n = {}", n);

    n = 0;
    loop {
        n += 1;
        if n == 100 {
            // stop action
            break;
        }
        if n % 2 == 0 {
            // stop action
            println!("偶数 {}", n);
            continue;
        }
    }
}
