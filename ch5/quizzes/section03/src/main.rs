// question 1
// impl Vec<32> {
//     fn len(&self) -> usize {
//         /* ... */
//     }
// }

/*
v = &mut Vec<i32>;
// try:
v.len();
compiles fine... &mut refernce is implicitly reborrowed as an & reference:
v.len() desugars to Vec::len(&*v) // a valid re-borrow of v
because the method contains a immutable refence to the struct which is the same as saying &*v

One would be unable to pass v directly to Vec::len since &self specifically refers to immutable references
*/

// question 2
struct Point(i32, i32);
impl Point {
    fn incr_v1(mut self) { self.0 += 1; }
    fn incr_v2(&mut self) { self.0 += 1; }
}
// which method is more idiomatic?
// clearly v2 because it does not consume ownership of Point


// question 3
impl Point {
    fn incr_x(&mut self) {
        self.0 += 1;
    }
}

// question 4
struct C_Point {
    x: i32,
    y: i32,
}

impl C_Point {
    fn get_x(&mut self) -> &mut i32 {
        return &mut self.x;
    }
}

fn main() {
    println!("Q3 Code");
    let mut p = Point(0,0);
    p.incr_x();
    println!("{}", p.0)
    // compiles

    println!("Q4 Code");
    let mut p = C_Point { x:1, y:2 };
    let x = p.get_x();
    *x += 1;
    println!("{} {}", *x, p.y);

    // this doesn't compile because get_x borrows a mutable reference to p. thus p cannot be used until x is dead (not used anymore)
    // therefore reading x and p.y in the same line is an ownership error...

}
