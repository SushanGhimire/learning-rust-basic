fn main() {
    let mut x = 5;
    const LET_X_VAR:u32= 60*10*20;
    println!("Value of x is: {x}");
    x= LET_X_VAR;
    println!("Value of x is: {x}");

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

}
