use std::fs;

fn main() {
    let data: String = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut max_sum: i32 = 0;
    let mut sec_sum: i32 = 0;
    let mut thi_sum: i32 = 0;
    let mut block_sum: i32 = 0;
    
    for line in data.lines() {
        if line == "" {
            if block_sum > max_sum {
                thi_sum = sec_sum;
                sec_sum = max_sum;
                max_sum = block_sum;
            }
            block_sum = 0;
        }
        else {
            block_sum += line.parse::<i32>().unwrap();
        }
    }

    println!("Max: {}, Sum of Top Three: {}", max_sum, max_sum + sec_sum + thi_sum);
}
