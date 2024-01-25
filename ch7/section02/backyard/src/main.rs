use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("Hello, garden!");
    let plant = Asparagus {
        length: 12,
        taste: String::from("bitter"),
    };
    println!("I'm growing {:?}!", plant);
}
