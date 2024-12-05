
fn main() {
    println!("Hello, world!");
    read_input();
}

fn read_input() {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    let inputs = std::fs::read_to_string("input.txt").unwrap();

    for lines in inputs.trim().lines().into_iter() {
        let parts: Vec<&str> = lines.split_whitespace().collect();

        if parts.len() != 2 {
            println!("{:?}", parts);
            panic!("parts must be equal to 2");
        }
        
        let left_value = parts[0].parse::<i64>().expect("it must be an i32 type");
        let right_value = parts[1].parse::<i64>().expect("it must be an i32 type");

        left.push(left_value);
        right.push(right_value);
    }
    left.sort();
    right.sort();

    let mut total_distance = 0;
    for (l,r) in left.iter().zip(right.iter()) {
            let distance = (r-l).abs();
            
            total_distance += distance;
    }
    println!("{}", total_distance);
}
