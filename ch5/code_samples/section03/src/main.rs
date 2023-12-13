/*
iterating on s2, we now refactor the area fn into a struct method with impl
*/

#[derive(Copy, Clone, Debug)]
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

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other:Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);
        *self = max;
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

    // methods and ownership
    println!("Methods and Ownership...");
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };

    println!("{}", rect.area());
    // let other_rect = Rectangle {
    //     width: 1,
    //     height: 1,
    // };

    // this will move the struct because the struct does not implement Copy; rect will lose all permissions...
    // let max_rect = rect.max(other_rect);

    // set_width requires write permissions, meaning the variable must be declared as mut
    rect.set_width(1);

    // let rect_ref = &rect;

    // rect_ref.set_width(2); // rect_ref is an immutable reference to a mutable var, so this can't work.
    // proper way to do this would be:
    // let mut rect_ref = &mut rect;


    /*
        Good Moves and Bad Moves
        Rectangle struct will have too derive copy and clone to accomplish this
    */
    let mut rect = Rectangle { width: 0, height: 1 };
    let other_rect = Rectangle { width: 1, height: 0 };
    rect.set_to_max(other_rect);
}
