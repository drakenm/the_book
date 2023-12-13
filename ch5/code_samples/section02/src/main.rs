// fn main() {
//     println!("Hello, world!");

//     /* 
//     * first way to write the program for area
//      */
//     // let width1 = 30;
//     // let height1 = 50;

//     // println!(
//     //     "The area of the rectangle is {} square pixels...",
//     //     area(width1, height1)
//     // );

//     /* 
//     * second way to write the program for area
//      */

//     let rect1 = (30, 50);
//     println!(
//         "The area of the rectangle is {} square pixels...",
//         area(rect1)
//     );
// }

// // fn area( width: u32, height: u32) -> u32 {
// //     return width * height;
// // }

// fn area(dimensions: (u32, u32)) -> u32 {
//     return dimensions.0 * dimensions.1;
// }


/*
Refactor using structs
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let w1: u32 = 30;
    let h1: u32 = 50;
    let rect1 = Rectangle {
        width: w1,
        height: h1,
    };

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(w1 * scale),
        height: h1,
    };

    dbg!(&rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}