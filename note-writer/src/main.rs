use std::collections::HashMap;
use std::error::Error;
use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use chrono::Local;

#[derive(Debug, Clone)]
pub struct Note {
    title: String,
    content: String,
    created_at: String,
}

type ContextResult = Result<(), Box<dyn Error>>;

const WRITE: &str = "w";
const OPEN: &str = "o";
const CLOSE: &str = "c";
const QUIT: &str = "q";
const LIST: &str = "l";
const NOTES_DIR: &str = "./notes";

fn main() {
    let mut notes: HashMap<String, Note> = HashMap::new();


    loop {
        print!("{esc}c", esc = 27 as char); // Clear the terminal screen
        println!("+--------------------------------------------------------------+");
        println!("| Note Writer - Commands                                       |");
        println!("+--------------------------------------------------------------+");
        println!("| (L) List notes | (W) Write | (O) Open | (C) Close | (Q) Quit |");
        println!("+--------------------------------------------------------------+");

        let mut command = String::new();

        match io::stdin().read_line(&mut command) {
            Ok(_) => {}
            Err(_) => {
                // TODO
            }
        }

        let res = match command.trim().to_lowercase().as_str() {
            WRITE => {
                write_context(&mut notes)
            }
            LIST => {
                list_context(&mut notes)
            }
            QUIT => {
                println!("+----------+");
                println!("| Goodbye! |");
                println!("+----------+");
                break
            }
            _ => Ok(())
        };

        if let Err(e) = res {
            let error_message = format!("| Error: {:?}", e);
            let title = "| Program Error";
            let mut error_len = error_message.len() + 2;
            if error_len < title.len() {
                error_len = title.len();
            }

            let line = fill("-", error_len, 0);

            println!("+{}+", line);
            println!("{}{} |", title, fill(" ", error_len, title.len()));
            println!("+{}+", line);
            println!("{}{} |", error_message, fill(" ", error_len, error_message.len()));
            println!("+{}+", line);
            println!();
            break
        }
    }
}

fn fill(c: &str, max_size: usize, actual_size: usize) -> String {
    if (actual_size >= max_size) {
        return "".to_string();
    }

    let mut filler =  String::new();
    for _ in actual_size..max_size {
        filler += c;
    }

    filler
}

fn write_context(notes: &mut HashMap<String, Note>) -> ContextResult {
    println!("+-----------------+");
    println!("| Write your note |");
    println!("+-----------------+");

    let mut content = String::new();
    let mut title = String::new();

    if let Err(e) = io::stdin().read_line(&mut content) {
        return Err(Box::new(e));
    }

    println!("+---------------------+");
    println!("| Write the note name |");
    println!("+---------------------+");

    if let Err(e) = io::stdin().read_line(&mut title) {
        return Err(Box::new(e))
    }

    let note = Note {
        title: title.trim().to_string(),
        content: content.trim().to_string(),
        created_at: Local::now().to_rfc3339(),
    };

    notes.insert(title.trim().to_string(), note.clone());
    
    let full_file_name = format!("{}/{}.txt", NOTES_DIR, title);
    let filepath = Path::new(full_file_name.as_str());

    match File::create(filepath.to_str().unwrap()) {
        Ok(mut file) => {
            let data = note.content.as_bytes();

            if let Err(e) = file.write_all(data) {
                return Err(Box::new(e));
            }
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }

    // TODO: Escrever aquivo com nota

    Ok(())
}

fn list_context(notes: &mut HashMap<String, Note>) -> ContextResult {
    if notes.is_empty() {
        // TODO -> Ler e indexar arquivos
    }

    let mut index = 0;

    notes.iter().for_each(|(title, _)| {
        println!(" - ({}) {}", index, title);
        index += 1;
    });

    Ok(())
}