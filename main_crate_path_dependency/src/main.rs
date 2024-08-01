fn main() {
    println!("Hello, world!");
    
    // uses the crate containing the warning
    dbg!(crate_with_warnings::add(10, 4));
}
