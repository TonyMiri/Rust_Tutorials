fn main() {
    //Mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //Shadowing. Redefine using let again
    let y = 5;
    let y = y + 1;
    
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    //Changing types with shadowing. Cannot do this with 'mut'
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of y is {y}");

    //Compiler can guess data types, but sometimes needs explicit
     let guess: u32 = "42".parse().expect("Not a number!");
}