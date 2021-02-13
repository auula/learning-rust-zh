fn main() {
    say_hi();
    assert_eq!(assert_sum(),sum());
    let  name = "Jarvib";
    edit_name(name);
    println!("main() Your name is {}",name);

    let mut no:i32 = 22;
    println!("The value of no is:{}",no);
    mutate_no_to_zero(&mut no);
    println!("The value of no is:{}",no);
}

fn say_hi() {
    println!("👋 Hello!");
}


fn sum() -> i8 {
    return 5 + 5;
}

fn assert_sum() -> i8 {
    5 + 5
}

fn edit_name(mut name:&'static str){
    name = "Jarvib Ding";
    println!("edit_name() Your name is {}",name)
}

fn mutate_no_to_zero(param_no:&mut i32){
    *param_no = 0; //解引用操作
 }