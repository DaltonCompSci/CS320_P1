use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap().skip(1);
    for path in paths {
        println!("Names of the files are: {}", path.unwrap().path().display())
    }
}
