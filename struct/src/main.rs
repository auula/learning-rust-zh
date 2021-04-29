struct UserInfo {
    name: String,
    age: u8,
    is_man: bool,
    weight: f32,
}

impl UserInfo {
    fn gender(&self) -> &'static str {
        if self.is_man {
            return "is a boy. ";
        }
        "is a girl."
    }
    fn say_hi(&self) {
        println!(
            "Hi, my name is {},age is {} and weight is {} kg,{}",
            self.name,
            self.age,
            self.weight,
            self.gender()
        );
    }
}

fn main() {
    let user = UserInfo {
        name: String::from("Jarvib Ding"),
        age: 0x16,
        is_man: true,
        weight: 67.8,
    };
    // Hi, my name is Jarvib Ding,age is 22 and weight is 67.8 kg,is a boy.
    user.say_hi()
}
