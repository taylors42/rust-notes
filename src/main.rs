use std::io;
use rand::Rng;
fn main() {
    mdc();
}
fn mdc(){
    let mut user_input = String::new();
    let mut second_user_input = String::new();
    println!("Digite o primeiro numero: ");
    io::stdin()
    .read_line(&mut user_input)
    .expect("err");
    println!("Digite o segundo numero: ");
    io::stdin()
    .read_line(&mut second_user_input)
    .expect("err");
    let primos: [i32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
    let mut divisores: Vec<i32> = Vec::new();
    let mut result: i32 = 0;
    let mut div1: i32 = convert_to_int(&user_input);
    let mut div2: i32 = convert_to_int(&second_user_input);
    while div1 > 1 || div2 > 1 {
        for index in &primos{
            if div1 % index == 0 && div2 % index == 0 && div1 > 0 && div2 > 0 {
                divisores.push(*index);
                div1 = div1 / index;
                div2 = div2 / index;
            }
            else if div1 % index == 0 && div1 > 0{
                div1 = div1 / index;
            }
            else if div2 % index == 0 && div2 > 0{
                div2 = div2 / index;
            }
        }
    }
    println!("######################");
    for i in 1..divisores.len(){
        result = divisores[i - 1] * divisores[i];
    }
    println!("{}", result)
}

fn media_aluno(){
    let mut user_input = String::new();
    println!("Quantas medias serão analisadas?\n");
    io::stdin()
    .read_line(&mut user_input)
    .expect("err");
    let mut num: i32 = convert_to_int(&user_input);
    println!("{num}");
    let mut notas: Vec<i32> = Vec::new();
    while num >= 1 {
        let mut nota1 = String::new();
        let mut nota2 =  String::new();
        let mut nota3 = String::new();

        println!("Digite a primeira nota \n");
        io::stdin().read_line(&mut nota1).expect("err");
        println!("Digite a segunda nota \n");
        io::stdin().read_line(&mut nota2).expect("err");
        println!("Digite a terceira nota \n");
        io::stdin().read_line(&mut nota3).expect("err");

        let int_nota1 = convert_to_int(&nota1);
        let int_nota2 = convert_to_int(&nota2);
        let int_nota3 = convert_to_int(&nota3);

        notas.push((int_nota1 + int_nota2 + int_nota3) / 3);
        num -= 1;
    }
    println!("essas foram as medias das notas:");
    for index in notas {
        if index < 3 {
            println!("a nota foi {} e foi reprovada", index);
        }
        else {
            println!("a nota foi {} e foi aprovada", index);
        }
    }

}

fn calc_factorial(){
    let mut factorial_input = String::new();
    println!("Write a number:");
    io::stdin()
    .read_line(&mut factorial_input)
    .expect("Error reading input");

    let num: i32 = convert_to_int(&factorial_input);
    let mut factorial: i32 = 1;

    for i in 1..=num {
        factorial *= i;
    }

    println!("{}", factorial);
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
