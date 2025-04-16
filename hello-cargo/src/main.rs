use std::io;

fn main() {
    println!("Please input a name");

    // Declarando uma variável mutável, por padrão elas são imutáveis
    let mut name = String::new();

    // &mut -> Indica que é uma referência de um valor mutável
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to get user input");

    if name.trim().is_empty() {
        name = String::from("world");
    }

    println!("Hello, {}!", name.trim());
}
