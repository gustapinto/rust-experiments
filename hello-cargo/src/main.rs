mod utils;

use utils::name::{get_name_from_user_input, name_or_default};

fn main() {
    let input_name = get_name_from_user_input()
        .expect("Failed to get username");
    let name = name_or_default(input_name);

    println!("Hello, {}!", name.trim());
}
