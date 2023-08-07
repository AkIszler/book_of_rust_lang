fn hello_function(){
    println!("Hello world!");
    println!("I am a function");
}

fn funcy_with_a_parma(x: i32){
    println!("the value of x is {}", x);

}

fn printed_label(value: i32, label: &str) {
    println!("the the number of {} in a cup is {}{}",label, value,label);
}

fn five() -> i32 {
    5
}

fn main() {

    let yvar = {
        let x = 5;
        x + x // 10
    };
    let xvar = {
        let y = 7 as f32; // changing to float to get proper value 
        y * y / 2.0 // without f32 cast, output would be 24 
    };

    println!("The value of y is: {yvar}");
    println!("The value of x is: {xvar}");

let fives = five();
println!("The value of fives is: {fives}");

hello_function();

funcy_with_a_parma(10); // will pass 10 to x and print the line

printed_label(16, "oz"); //

let xdemo = plus_one(5);

println!("{}", xdemo);

}

fn plus_one(n: i32) -> i32 {
    n + 1
}