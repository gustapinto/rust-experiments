use std::io;
use std::io::Error;

const DEFAULT_NAME: &str = "world";

pub fn name_or_default(name: String) -> String {
    if name.trim().is_empty() {
        return DEFAULT_NAME.to_string();
    }

    name.trim().to_string()
}

pub fn get_name_from_user_input() -> Result<String, Error> {
    println!("Please input a name");

    let mut name = String::new();

    // &mut -> Indica que é uma referência de um valor mutável
    // match <value> -> Lida com o resultado, e possível erro
    match io::stdin().read_line(&mut name) {
        Ok(_) => Ok(name), // Retorna um resultado posítivo com o nome
        Err(e) => Err(e), // Retorna um resultado de erro
    }
}

// Declara o módulo de testes
#[cfg(test)]
mod name_tests {
    use super::*;

    #[test] // Declara um teste
    fn test_name_or_default_should_return_the_name_when_valid() {
        let expected = "foobar".to_string();
        let actual = name_or_default("foobar".to_string());

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_name_or_default_should_return_the_default_on_empty_name() {
        let expected = DEFAULT_NAME.to_string();
        let actual = name_or_default("".to_string());

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_name_or_default_should_return_the_default_on_blank_name() {
        let expected = DEFAULT_NAME.to_string();
        let actual = name_or_default(" ".to_string());

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_name_or_default_should_return_the_default_on_empty_name_with_line_breaks() {
        let empty_names = vec!["\n", "\r\n"];

        for empty_name in empty_names {
            let expected = DEFAULT_NAME.to_string();
            let actual = name_or_default(empty_name.to_string());
    
            assert_eq!(expected, actual);
        }
    }
}
