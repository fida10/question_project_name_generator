use std::io::{self, Write};

fn main() {
    let mut chapter_name = String::new();
    let mut lesson_name = String::new();

    print!("Enter chapter name: ");
    io::stdout().flush().unwrap(); // Making sure the print! macro outputs immediately
    io::stdin().read_line(&mut chapter_name).expect("Failed to read line");
    chapter_name = chapter_name.trim().replace(|c: char| !c.is_alphanumeric(), "");

    print!("Enter lesson name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut lesson_name).expect("Failed to read line");
    lesson_name = lesson_name.trim().replace(|c: char| !c.is_alphanumeric(), "");

    let combined = format!("{}_{}", chapter_name, lesson_name).to_lowercase();
    println!("Combined string: {}", combined);
}
