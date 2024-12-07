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


    // part b
    let mut similarity_value: i32 = 0;

    for i in 0..left_list.len() {
        let occurences = get_accurences_in_right_list(&right_list, left_list[i]);
        if occurences > 0 {
            similarity_value += left_list[i] * occurences;
        }
    }

    println!("The total difference for part 1 is: {}", total_distance);
    println!("The similarity value for part 2 is: {}", similarity_value);
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

/// Return the amount of times the given number is in the list
fn get_accurences_in_right_list(list: &Vec<i32>,number: i32) -> i32 {
    return list.iter().filter(|&x| *x == number).count() as i32
}


#[allow(dead_code)]
#[test]
fn test_find_occurences() {
    let test: Vec<i32> = vec![1, 2, 0, 4, 5, 0, 7, 8, 9, 10];
    assert_eq!(get_accurences_in_right_list(&test, 0), 2);
    assert_eq!(get_accurences_in_right_list(&test, 11), 0);
}
