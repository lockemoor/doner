use std::{env, fs::File, io::{self, BufRead}};
use std::path::Path;

extern crate walkdir;
use walkdir::WalkDir;


fn main() {
    
    let args: Vec<String> = env::args().collect();
    let first_arg: &str = &args[1];

    match first_arg{
        "--repo" | "-r" => find_todos_in_files(&args[2]),
        _ => find_todos(&args[1])
    }
}

fn find_todos_in_files(root: &String) -> () {

    let file_list: Vec<String> = list_files(root);

    for filepath in file_list.iter() {
        find_todos(filepath);
    }
}

fn list_files(root: &String) -> Vec<String>{

    let mut file_paths: Vec<String> = Vec::new(); 

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file() {
            file_paths.push(entry.path().display().to_string());
        }
    }
    return file_paths;
}

fn find_todos(filepath: &String) -> () {
    
    if let Ok(lines) = read_by_line(filepath){
        for line in lines.flatten().enumerate() {
            if line.1.contains("TODO") {
                println!("In : {:?} at line: {} - {}", filepath, line.0+1 , line.1);
            }
    }
        
    }
}

fn read_by_line<P>(filepath:P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {

        let file = File::open(filepath)?;
        Ok(io::BufReader::new(file).lines())
}