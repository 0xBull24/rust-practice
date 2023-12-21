// variables have scope - either global or block scope
// variables must be initialized before used - compiler is always looking for 
fn main() {
    let x = 5;
    {
        let y = 00;
        println!("{}, {}", x, y);
    }
    {
        // shadowing a variable is naming a same variable with a different value in a different block
        let x = 3;
        println!("{}", x);
    }
    println!("{}",x);
    // println!("{}, {}", x, y);  throws error since y is inside block above. uncomment and run
}
