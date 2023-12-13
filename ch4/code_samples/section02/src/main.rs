fn main() {
    let name = vec![String::from("Ferris"), String::from("Bueller")];
    let f = stringify_name_with_title(&name);
    println!("You can call me {}", f);

    let f2 = stringify_name_with_title2(&name);
    println!("AND YOU CAN CLONE ME RESPONSIBLY... {}", f2);
}

// clone copies every string in the input...
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    return full;
}

// rewritten to avoid unnecessary copies by adding the suffix later...
fn stringify_name_with_title2(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    return full;
}