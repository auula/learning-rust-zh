fn main() {
    let mut year: [i32; 4] = [1999, 2019, 2020, 2021];
    // let arr = [10,20,30,40];
    println!("array is {:?}", year);
    // len() 数组长度
    println!("array size is :{}", year.len());

    for index in 0..year.len() {
        println!("index: {} , value: {}", index, year[index]);
    }

    for value in year.iter() {
        println!("value is: {}", value);
    }

    updated_by_index(3, 2021, &mut year);
    println!("updated of {:?}", year)
}

// 通过下标修改某个元素的值
fn updated_by_index(index: usize, value: i32, arr: &mut [i32; 4]) {
    for i in 0..index {
        arr[i] = value;
    }
}
