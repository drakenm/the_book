/*
iterating on s2, we now refactor the area fn into a struct method with impl
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }
}

fn main() {
    let w1: u32 = 30;
    let h1: u32 = 50;
    let w2: u32 = 10;
    let h2: u32 = 40;
    let w3: u32 = 60;
    let h3: u32 = 45;

    let rect1 = Rectangle {
        width: w1,
        height: h1,
    };

    let rect2 = Rectangle {
        width: w2,
        height: h2,
    };

    let rect3 = Rectangle {
        width: w3,
        height: h3,
    };

    println!(
        "The are of the rectangle is {} square pixels...",
        rect1.area()
    );

    println!("can rec1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rec1 hold rect3? {}", rect1.can_hold(&rect3));

    // understanding differences and similarities of method calls vs function calls
    let r = &mut Box::new(Rectangle {
        width: 1,
        height:2,
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);
}
