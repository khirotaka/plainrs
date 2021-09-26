use std::fs;
use std::io::prelude::*;
use std::path::Path;
use clap::{App, Arg};


fn read_file(path: &str) -> String {
    let content: String = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(error) => panic!("Error Opening File Failed: {}", error)
    };

    content
}

fn write_file(content: &str, filename: &str) {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match fs::File::create(&path) {
        Ok(file) => file,
        Err(error) => panic!("Error Couldn't create: {}: {}", display, error.to_string()),
    };

    let mut content = content.to_string();
    content.push('\n');
    
    match file.write_all(content.as_bytes()) {
        Ok(_) => (),
        Err(error) => panic!("Error Cound't write to {}: {}", display, error.to_string()),
    }
}
    

fn main() {
    let matches = App::new("Text Viewer & Writer")
        .version("0.1.0")
        .author("Hirotaka Kawashima")
        .arg(Arg::new("read")
            .short('r')
            .long("read")
            .value_name("FILE")
        )
        .arg(Arg::new("write")
            .short('w')
            .long("write")
            .value_names(&["FILE", "TEXT"])
        )
        .get_matches();
    
    if let Some(file_path) = matches.value_of("read") {
        let content = read_file(file_path);
        println!("{}", content);
    }
    
    if let Some(args) = matches.values_of("write") {
        let args = args.collect::<Vec<&str>>();
        write_file(args[1], args[0]);
    }
}
