use std::{env, fs::File, io::{self, BufRead}};
use std::path::Path;


fn main() {
    
    let args: Vec<String> = env::args().collect();
    let first_arg: &str = &args[1];

    match first_arg{
        "--repo" | "-r" => println!("{}", &args[2]),
        _ => find_todos(&args[1])
    }

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