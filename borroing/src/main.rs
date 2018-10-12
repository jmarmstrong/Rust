fn main() {
    let s = String::from("hello");

    let _r1 = &s; // no problem
    let _r2 = &s; // no problem
    println!("{}", _r1);
}
