use std::fs::File;
use std::io::Read;

fn main() {
    println!(
        "{}",
        read_file_content("/Users/ding/Desktop/learning-rust-zh/io_file/content.txt")
    )
}

fn read_file_content(path: &str) -> String {
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(error) => {
            panic!("problem openting the file {:?}", error)
        }
    };
    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => {
            panic!("problem read file content {:?} ", e)
        }
    }
}
