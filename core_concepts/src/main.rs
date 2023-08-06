// in this section I will be going over core conepts of Rust Lang
// as well as exploring mutiblity 





fn main() {

let x = 5;
println!("the value of x is: {}", x);
// this wont work
// x = 6;
// println!("the value of x is: {}", x);


let y = 5;
println!("the value of y is: {}", y);
// this works because I am using the let keyword
let y = 6;
println!("the value of y is: {}", y);

let mut y = 5;
println!("the value of y is: {}", y);
// this works because I am using the mut keyword
y = 6;
println!("the value of y is: {}", y);


//----------------------------------------------------------------
// using const to hold static variables that dont change.
const MIN_IN_HOURS: usize = 60;
const SEC_IN_HOURS: usize = MIN_IN_HOURS * 60;

let hours = 10;

let answer = hours * SEC_IN_HOURS;
println!("{}", answer);

// let text:u8 = 256; <=== will overflow, wont compile


}
