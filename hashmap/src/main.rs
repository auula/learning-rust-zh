use std::collections::HashMap;

fn main() {
    //通过泛型创建一个map
    let mut map: HashMap<&str, i32> = HashMap::new();

    map.insert("Java", 100);
    map.insert("Golang", 80);
    map.insert("Rust", 70);

    println!("{:?}", map);

    // 交换Java和Rust
    if let Some(ele) = map.insert("Java", 90) {
        map.insert("Rust", ele);
    }
    println!("{:#?}", map);
    // key有值的话不做操作，反之插入值
    map.entry("JavaScript").or_insert(86);

    //遍历map
    for (i, v) in map.iter_mut() {
        println!("index = {}  value = {}", i, v);
    }
    //println!("Hello {0:1$}", 5, "x");

    // 移除指定的key
    if map.contains_key("Java") {
        map.remove("Java");
    }
    println!("{:#?}", map);

    let key: String = String::from("key");

    let mut value: String = String::from("value");

    let mut map_string: HashMap<&String, &String> = HashMap::new();

    // 引用和借用
    map_string.insert(&key, &mut value);

    //value.clear();

    println!("value = {:?}", map_string.get(&key))
}
