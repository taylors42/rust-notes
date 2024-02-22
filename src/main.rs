use std::io;
use rand::Rng;
fn main() {
    sum_all_digits();
}

fn hello_world(){
    println!("hello world");
}

fn say_user_input(){
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("err");
    println!("{user_input}");
}

fn flux_control(num1: i32, num2: i32){
    if num1 > num2 {
        println!("{num1} is greater than {num2}");
    }
    else if num1 < num2 {
        println!("{num1} is less than {num2}");
    }
    else {
        println!("{num1} is equal to {num2}");
    }
}

fn convert_to_int(data_input: &String) -> i32 {
    let x: i32 = data_input.trim().parse::<i32>().unwrap(); //unwrap will panic if there is an error and get the value
    x // return the value
}

fn flux_control_by_user(){
    let mut num1: String = String::new();
    let mut num2: String = String::new();
    io::stdin().read_line(&mut num1)
    .expect("fail");
    io::stdin().read_line(&mut num2)
    .expect("fail");

    if convert_to_int(&num1) > convert_to_int(&num2){
        println!("{num1} is greater than {num2}");
    }
    else if convert_to_int(&num1) < convert_to_int(&num2) {
        println!("{num1} is less than {num2}");
    }
    else {
        println!("{num1} is equal to {num2}");
    }
}

fn sum_all_digits(){
    let mut total: i32 = 0;
    let mut initial_value = String::new();
    io::stdin().read_line(&mut initial_value)
    .expect("err");
    let mut value: i32 = convert_to_int(&initial_value);

    while value != 0 {
        let mut r: i32 = value % 10;
        total = total + r;
        value = value / 10;
    }
    println!("{total}");
}
