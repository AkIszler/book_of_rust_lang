use std::io;

fn main() {
    
let a = [1, 2, 3, 4, 5];

println!("Please pick an array index 0-4");

let mut idex = String::new();

io::stdin().read_line(&mut idex)
.expect("Error reading line");

let idex: usize = idex.trim()
.parse().expect("Error parsing line");

let element = a[idex];


println!("{} is the value at index {}", element, idex);

// more to come later

let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

println!("The result is {result}");

loopedloops();

}

fn loopedloops(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}