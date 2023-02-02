fn main() {
    //interger
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    //float
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{x}-{y}");

    //boolean
    let t: bool = true;
    println!("{t}");

    //string
    let new_string = "Hello world";
    println!("{new_string}");

    //character
    let new_char: char = 'Z';
    println!("{new_char}");

    //tuple
    let tup: (i32, f64, u8) = (300, 1.0, 1);
    let (x, _y, _z): (i32, f64, u8) = tup;
    println!("{x}");
    let tup_x = tup.0;
    println!("{tup_x}");

    //array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{first}");
    println!("{second}")
}
