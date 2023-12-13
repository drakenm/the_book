fn main() {
    println!("Fixing an Unsafe Program: Returning a Reference to the Stack");
    let s = return_a_string();
    println!("\n{}", s);
    let s = return_a_string2();
    println!("string literalsss...\n{}", s);

    let s = return_a_string3();
    println!("reference-counted pointer string...\n{}", s);

    let mut s = String::from("WAKA-FLAKA-FLIM-FLAM!");
    return_a_string4(&mut s);
    println!("mutable reference string...\n{}", s);
}

// moves ownership of the string out of the function
fn return_a_string() -> String {
    let s = String::from("NAKA-WAKA-FIJI!");
    return s;
}

// returns a string literal, which is a reference to a static string
fn return_a_string2() -> &'static str {
    return "NAKA-NAKA-FIJI!";
}

// defer borro-checking to runtime by using garbage collection: use a reference-counted pointer
use std::rc::Rc;
fn return_a_string3() -> Rc<String> {
    let s = Rc::new(String::from("WAKA-FLAKA-FLIM-FLAM!"));
    return Rc::clone(&s);
}


// have caller provide a 'slot' to put the string into using as mutable reference
fn return_a_string4(output: &mut String) {
    return output.replace_range(.., "THE FLIM TO THE FLAM, SPAMMIN FLAM, LIKE WE SLAMMIN JAM!");
}