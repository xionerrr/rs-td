use std::{fs::File, path::Path};

pub fn open_test_file() {
    let path = Path::new("../mocks/preview.txt");
    let display = path.display();

    println!("{:?}", display);

    match File::open(&path) {
        Ok(file) => {
            println!("File opened successfully: {:?}", file);
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
}
