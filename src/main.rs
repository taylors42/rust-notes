use std::io; // <- importando biblioteca de io de dentro de std
fn main() {
    let name: &str = "taylor"; // <- variavel não mutavel
    let mut name1: &str = "taylor mutavel"; // <- variavel mutavel
    let mut vazio = String::new() ; // <- cria uma string vazia e new é um metodo da classe string
    println!("Thats my first script in RUST, and I'm {}", name);
    io::stdin().read_line(&mut vazio)
    //stdin is a method inside the io thats read a line
    //(&mut) indica que estamos recebendo um valor mutavel e 
    // que o read_line pode modificar o conteudo dentro do vazio
    //vazio é onde será armazenado o valor que o usuario digitar
        .expect("Falha ao ler entrada"); // <- caso de erro, retornar str
}
