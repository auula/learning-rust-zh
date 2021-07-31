// 矩形 泛型
#[derive(Debug)]
struct Rectangle<T> {
    width: T,
    hight: T,
}

// 实现泛型trait
impl<T> Rectangle<T> {
    pub fn widht(&self) -> &T {
        &self.width
    }
    pub fn hight(&self) -> &T {
        &self.hight
    }
}

// 接口特性
trait Geometry {
    fn area(&self) -> i32;
}

// 为i32类型的实现求面积特性
impl Geometry for Rectangle<i32> {
    fn area(&self) -> i32 {
        self.width * self.hight
    }
}

// 为rectangle实现i32方法
impl Rectangle<f32> {
    pub fn area(&self) -> f32 {
        self.width * self.hight
    }
}

fn option_add(x: Option<f32>, y: Option<f32>) -> Option<f32> {
    return if x.is_none() && y.is_none() {
        None
    } else if x.is_none() {
        y
    } else if y.is_none() {
        x
    } else {
        Some(y.unwrap() + x.unwrap())
    };
}

struct Computer {
    brand: String,
}

trait USB2 {
    fn usb_2(&self);
}

trait USB3 {
    fn usb_3(&self);
}

impl USB2 for Computer {
    fn usb_2(&self) {
        println!("{} impl usb2.0 ", &self.brand)
    }
}

impl USB3 for Computer {
    fn usb_3(&self) {
        println!("{} impl usb3.0 ", &self.brand)
    }
}

// usb(pc:impl USB3+USB2)
fn usb<T>(pc: T)
where
    T: USB3 + USB2,
{
    pc.usb_2();
    pc.usb_3();
}

fn main() {
    let rtl = Rectangle {
        width: 6,
        hight: 12,
    };

    // method `area` not found for this
    println!("area = {}", rtl.area());
    println!("widht = {}", rtl.widht());
    println!("hight = {}", rtl.hight());
    println!("{:?}", rtl);

    let area_func = |g: &dyn Geometry| println!("area_func = {}", g.area());

    area_func(&rtl);

    println!("option_add = {:?}", option_add(Some(1f32), Some(9f32)));

    usb(Computer {
        brand: "Dell".to_string(),
    })
}
