use std::{env, fs::{self, File}, io::{self, BufRead}};
use std::path::Path;


fn main() {
    
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    if let Ok(lines) = read_by_line(file_path){
        for line in lines.flatten() {
            if line.contains("TODO") {
                println!("TODO Here: {} ", line);
            }
    }
        
    }
}

fn read_by_line<P>(filepath:P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {

        let file = File::open(filepath)?;
        Ok(io::BufReader::new(file).lines())
}