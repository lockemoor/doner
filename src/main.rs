use std::{env, fs::{self, File}, io::{self, BufRead}};
use std::path::Path;


fn main() {
    
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    if let Ok(lines) = read_by_line(file_path){

        let mut i: i32 = 0;

        for line in lines.flatten() {
            i += 1;
            if line.contains("TODO") {
                println!("In : {} at line: {} : {}", file_path, i, line);
            }
    }
        
    }
}

fn read_by_line<P>(filepath:P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {

        let file = File::open(filepath)?;
        Ok(io::BufReader::new(file).lines())
}