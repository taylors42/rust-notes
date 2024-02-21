use std::io;
use rand::Rng;
fn main() {
    hello_world();
    say_user_input();
}

fn hello_world(){
    println!("hello world");
}

fn say_user_input(){
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("err");
    println!("{user_input}");

}
