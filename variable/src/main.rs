// making variables is simple
// can use `let`, `static`, or `const` to define vars
fn main() {
    const BUNNIES: i8 = 2;
    println!("{}", BUNNIES);

    let mut fruit = "banana";
    println!("{}", fruit);
    
    // fruit = "apples";
    // in order to change a variable you must make it mutable
    fruit = "apples";
    println!("{}", fruit);

    const ALL_CAPS: &str = "BURGER";
    println!("{}", ALL_CAPS);

}