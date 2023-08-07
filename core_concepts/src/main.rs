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
    
    // tuples

    let new_tup:(i32, i32, i32) =(10,10,20);

    let (a,b,c) = new_tup;

    println!("{}",b);
    
    let q: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = q.0;

    let six_point_four = q.1;

    let one = q.2;

    let arr = [1,3,5,6,7,3,6];
    
    for i in 0..arr.len() {
        println!("{} - {}", (i+1),  arr[i]);
        }

    let three_arr = [3; 4];
    println!("{} {}", three_arr[0], three_arr[1]);
    
    let float = 2.3;
    let float2: f32 = 3.4;

    println!("{} \n{}", float, float2);

    }
    