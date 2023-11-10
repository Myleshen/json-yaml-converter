use std::{fs::{self, File}, io::Write};
use itertools::Itertools;
// JSON to YAML
// Get JSON file
// Parse JSON to rust object
// Check if valid JSON
// take key -> set it as top level
// inside key take it as secondary level and follow again
// if value is not an array or object then it needs to go only 1 level deep
// else it needs to go 1 level deep and start unwrapping those as well

const FILE_WRITE_ERROR: &str = "File not writable";
const INPUT_FILE_PATH_ERROR: &str = "Input file is not available";
const OUTPUT_FILE_PATH_ERROR: &str = "Output file cannot be created";

fn write_space(spaces: i32, mut file:&File) {
    let mut curr_space = 0;
    while curr_space < spaces {
        write!(file, " ").expect(FILE_WRITE_ERROR);
        curr_space+=1;
    }
}


fn main() {
    let file_path = std::env::args().nth(1).expect(INPUT_FILE_PATH_ERROR);
    let file_name = std::env::args().nth(2).expect(OUTPUT_FILE_PATH_ERROR);
    let content = fs::read_to_string(file_path).expect("Cannot Read File"); let mut modified_lines:Vec<String> = Vec::new();
    for line in  content.lines(){
        let mut line_string = String::from(line);
        if !line_string.contains("[") || !line_string.contains("]") {
            line_string.retain(|c| c != '"' && c != ',');
        }
        let white_space_trimmed = line_string.split_whitespace().collect::<String>();
        let line_splits = white_space_trimmed.split(":").collect::<Vec<&str>>();
        let print_line = line_splits.iter().join(",");
        modified_lines.push(print_line);
    }
    modified_lines.remove(0);

    let mut spacing = 0;
    let mut output_file = File::create(file_name).unwrap();
    for values in modified_lines {
        if values.contains("{") {
            let mut splitted_values = values.split(",");
            write_space(spacing, &output_file);
            writeln!(output_file, "{}:", splitted_values.nth(0).unwrap()).expect(FILE_WRITE_ERROR);
            spacing+=2;
        } else if values.contains("}") {
            spacing-=2;
        } else {
            let mut splitted_values = values.split(",");
            // println!("{:?}", splitted_values);
            if values.contains("[") || values.contains("]") {
                let mut key_mut = String::from(splitted_values.nth(0).unwrap());
                key_mut.retain(|c| c != '"');
                let key_name = key_mut.split_whitespace().collect::<String>();
                writeln!(output_file, "{}:", key_name).expect(FILE_WRITE_ERROR);
                let mut array_values: String = splitted_values.join(",").split_whitespace().collect();
                array_values.retain(|c| c != '[' && c != ']');
                let mut split_array_values = array_values.split(",").collect::<Vec<&str>>();
                split_array_values.remove(split_array_values.len()-1);
                // println!("{:?}", split_array_values);
                for array_elem in split_array_values {
                    write_space(2, &output_file);
                    writeln!(output_file, "- {}", array_elem).expect(FILE_WRITE_ERROR);
                }
            } else {
                write_space(spacing, &output_file);
                writeln!(output_file, "{}: {}", splitted_values.nth(0).unwrap(), splitted_values.nth(0).unwrap()).expect(FILE_WRITE_ERROR);
            }
        }
    }
}
