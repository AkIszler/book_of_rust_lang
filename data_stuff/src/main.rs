fn main() {

let mut s = String::from("hello"); // string representation

s.push_str(", people"); // pushing to string representation

println!("{s}");

let mut n = 5;

while n > 0 {
    s.push_str("!");
    n -= 1;
    }
    println!("{s}"); // prints Hello people after adding 5 "!" characters

    let change = change_length(&mut s);

    let len = calculate_length(&s);

    if len >= 20 {
        println!("{} is too long" , len);
    }
    else{
    println!("{len}"); // will trigger and print out 18
    }
}


fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change_length(some_string: &mut String){
    if some_string.len() <= 20{
    some_string.push_str("!");
    }
    else {
        while some_string.len() >= 21 {
        some_string.push_str("!");}
    }

}
