use std::fs;

pub fn read_file_lines(file_path:String)->String{
    let contents = fs::read_to_string(file_path).expect("Could not read {file_path}");
    return contents;
}