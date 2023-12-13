fn main() {
    println!("Hello, world!");


    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];
    // read permissions needed for the dereference operation on n_ref
    // this works
    let n: i32 = *n_ref;

    let v: Vec<String> = vec![String::from("hello world!?")];
    let s_ref: &String = &v[0];
    // v owns the String heap-allocated data, so v must be borrowed mutably
    // let s: String = *s_ref;

    // i32 does not own heap data - can be copied without a move
    // String does own heap data - cannot be copied without a move
    // &String does not own heap data - can be copied without a move

    // way of dealing with this:
    println!("s_ref: {}", s_ref); // avoid ownership and use immut ref
    let mut s: String = v[0].clone(); // clone it
    s.push('!'); // mutate the clone
    println!("s: {}", s); // use the clone

    let mut v: Vec<String> = vec![String::from("hello world!?")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("s: {}", s);
    assert!(v.len() == 0);

    // dealing with tuples and mutation
    let mut name = (
        String::from("Drake"),
        String::from("Rustacean")
    );

    let first = &name.0;

    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);

    // the above is safe but if we place &name into a fn, we can get into trouble. The borrow checker doesn't know that the fn won't mutate name. So we can't borrow name immutably and then mutate it later like this:
    fn get_first(name: &(String, String)) -> &String {
        &name.0
    }
    let first = get_first(&name);
    // the following mutation now fails to compile because the borrow checker doesn't know that get_first won't mutate name so write permissions are assumed to be revoked
    // name.1.push_str(", Esq.");

    // arrays:
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    *x += 1;
    println!("{a:?}");
    // this works
}

// dst.push could possibly deallocate the contents of dst, which could invalidate the reference returned by max_by_key.
// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();

//     for s in src{
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }

// we fix this by copying the length of the largest string into a local variable, and then using that variable in the loop. Since we do not borrow dst in the loop, we can safely call dst.push
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();

    for s in src{
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

