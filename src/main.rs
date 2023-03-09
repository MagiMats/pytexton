use pytexton::Controller;
use std::io;

fn main() {
    let mut user_input: String = String::new();

    println!("Write some text to lex");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let text_controller = Controller {
        text: user_input,
    };

    let parsed_text = text_controller.parse_text();
    println!("{:?}", parsed_text);
}