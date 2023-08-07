fn main() {

    // below is a handful of operations that can be used in Rust
    
let x = 7;
let y = 3;

let sum = x + y;
let diff = x - y;
let product = x * y;
let divison = x / y;
let remainderx = x & 2;
let remaindery = y & 2;

println!("{}", product);
println!("{}", sum);
println!("{}", remainderx);

    //booleans which are typical like in most other languages

let t = true;
let f = false;

if t == true {
    println!("true")
}
if f == false {
    println!("false")
}

    // another type of primitive is the char

    let c = "c"; // simple char
    let z: char = 'ℤ'; // explicit character type
    let heart = '😻';

    println!("{} {}", c, z);

    // more compond types 

    let five: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{}",five[2]);
    println!("{}",five[1]);

    for i in 0..five.len()  {
        print!("{}",i);
    }
}

