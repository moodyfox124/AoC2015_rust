use md5;

fn main() {
    let input = "yzbqklnj";

    let mut lowest_possible_number = 0;
    loop {
        let digest = md5::compute(format!("{}{}", input, lowest_possible_number));
        let hex = format!("{:x}", digest);
        println!("{}, index {}", hex, lowest_possible_number);
        if hex.starts_with("000000") {
            break;
        }
        lowest_possible_number += 1;
    }

    println!("Lowest possible number to generate MD5 hash with 5 leading zeros is {}", lowest_possible_number)
}
