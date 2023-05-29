// Advent of code Day 1
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn read_as_string() {
    let path: &Path = Path::new("inputs/day_1_input.txt");
    let mut file: File = File::create("outputs/day_1_output.txt").expect("Unable to create file");
    let data: String = fs::read_to_string(path).expect("Unable to read file");
    let mut sum = 0;
    let mut all_sum: Vec<i32> = Vec::new();

    for line in data.lines() {
        let token = line.split_whitespace().collect::<Vec<_>>();

        if token.is_empty() {
            all_sum.push(sum);
            sum = 0;
        } else {
            for i in 0..token.len() {
                let num = token[i].parse::<i32>().unwrap();
                sum += num;
            }
        }
    }
    // println!("{:?}", all_sum);

    // Write out result to .txt file for debugging
    file.write_all(format!("{:?}", all_sum.iter().max().unwrap()).as_bytes())
        .expect("Unable to write data");

    // sort all_sum
    all_sum.sort();

    all_sum.reverse();

    println!("{:?}", all_sum[0..3].to_vec().iter().sum::<i32>());

    // write out top 3 highest values
}

fn main() {
    read_as_string();
}
