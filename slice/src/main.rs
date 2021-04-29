fn main() {
    let key: &'static str = "sync_lock";
    if up_lock(key, 1) == 1 {
        // 设置超时
        expire(key, 30)
        // .....业务逻辑
    }
}

// 基于redis SETNX 和 EXPIRE 的实现，问题代码
fn up_lock(key: &'static str, num: i8) -> i8 {
    // ..... 上锁逻辑
    return 1;
}

fn expire(key: &'static str, num: i8) {
    // ... 自定义超时
}
