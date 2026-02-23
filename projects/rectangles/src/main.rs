#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold_other(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect: Rectangle = Rectangle {
        height: 40,
        width: 100,
    };
    let rect1: Rectangle = Rectangle {
        height: 30,
        width: 110,
    };
    let rect2: Rectangle = Rectangle {
        height: 20,
        width: 80,
    };

    dbg!(&rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold_other(&rect2));
    println!("Can rect hold rect2? {}", rect.can_hold_other(&rect2));
    println!("Can rect hold rect1? {}", rect.can_hold_other(&rect1));

    println!("{:?}", Rectangle::square(39));
}
