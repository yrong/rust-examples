// Dangling References
// Rust never gives you a dangling pointer.

fn main() {
    let refToNothing = _safe();
    // -> think about using the null pointer here! CRASH!

    println!("{}", refToNothing);
}

/*
fn dangle() -> &String {            // returns a ref to a String
    let s = String::from("hello");
    &s                              // return a ref to s
}*/

fn _safe() -> String {
    let s = String::from("hello");
    s
}