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

    updated_by_index(2, 2021, &mut year);
    println!("updated of {:?}", year);

    delete_by_index(1, &mut year)
}

// 通过下标修改某个元素的值
fn updated_by_index(index: usize, value: i32, arr: &mut [i32; 4]) {
    arr[index] = value;
}

// 通过下标删除某个元素
fn delete_by_index(index: usize, arr: &mut [i32; 4]) {
    // 新数组，长度为原始数组减去 1
    let mut new: [i32; 3] = [-1; 3];
    for i in 0..new.len() {
        if index <= 0 || index >= arr.len() {
            println!("下标越界！")
        }
        if i < index {
            new[i] = arr[i];
        } else {
            new[i] = arr[i + 1]
        }
    }
    // return new;
    println!("new arr {:?}", new)
}
