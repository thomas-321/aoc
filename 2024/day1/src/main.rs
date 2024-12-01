use std::fs::read_to_string;


fn main() {
    println!("day 1 - part 1");

    let input = read_to_string("src/inputa.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let (left_list, right_list) = split_list(&lines);

    let mut left_list = vecstr_to_veci32(left_list);
    let mut right_list = vecstr_to_veci32(right_list);
    
    left_list.sort();
    right_list.sort();

    assert_eq!(left_list.len(), right_list.len());

    let mut total_distance = 0;
    for i in 0..left_list.len() {
        total_distance += get_difference(left_list[i], right_list[i]);
    }

    print!("The total difference for part 1 is: {}", total_distance);
}

/// Function takes a list of strings
/// returns a tuple of two lists with the first two string slices 
/// seperated by whitespaces
fn split_list<'a>(list: &'a Vec<&str>) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut left_list: Vec<&str> = Vec::new();
    let mut right_list: Vec<&str> = Vec::new();

    for line in list {
        let mut numbers = line.split_whitespace();
        left_list.push(numbers.next().unwrap());
        right_list.push(numbers.next().unwrap());
    }

    return (left_list, right_list); 
}

/// takes a list of &str and parses it to a list of i32
/// returns the parsed list
fn vecstr_to_veci32(str_list: Vec<&str>) -> Vec<i32> {
    return str_list.iter().map(|x| x.parse::<i32>().unwrap()).collect();
}

/// takes two i32 values and returns the difference between them
/// returns the difference
fn get_difference(a: i32, b: i32) -> i32 {
    return (a - b).abs();
}
