use std::io::{self, Write};

fn main() {
    let mut chapter_name = String::new();
    let mut rerun = true;

    while rerun {
        let mut lesson_name = String::new();

        print!("Enter chapter name OR enter \'exit\' to exit; leave blank and press enter to use previous chapter name: ");
        io::stdout().flush().unwrap(); // Making sure the print! macro outputs immediately
        let mut temp_chap_name: String = String::new();
        io::stdin()
            .read_line(&mut temp_chap_name)
            .expect("Failed to read line");
        if temp_chap_name.eq("\n") {
            println!("Using previous chapter name: {}", chapter_name)
        } else if temp_chap_name.eq("exit\n") {
            println!("Exiting!");
            rerun = false;
            continue;
        } else {
            chapter_name = temp_chap_name;
        }

        chapter_name = chapter_name
            .trim()
            .replace(|c: char| !c.is_alphanumeric(), "");

        print!("Enter question name: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut lesson_name)
            .expect("Failed to read line");
        lesson_name = lesson_name
            .trim()
            .replace(|c: char| !c.is_alphanumeric(), "");

        let combined = format!("{}_{}", chapter_name, lesson_name).to_lowercase();
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("Combined string: {}", combined);
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    }
}
