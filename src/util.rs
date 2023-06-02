use std::fs;

pub fn get_content(path: &str) -> Vec<String> {
    let file = fs::read_to_string(path).expect("file not found");

    file.lines().map(|x| x.to_string()).collect::<Vec<String>>()
}
