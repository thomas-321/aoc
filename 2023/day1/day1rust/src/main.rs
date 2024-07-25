use std::fs::read_to_string;



fn main() {
    let filename = "src/input.txt";
    let num_vector = ["0", "1", "2", "3", "4", "5" ,"6", "7", "8", "9",
                      "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].to_vec();

    println!("value {}", find_first_number_in_string("itestone", &num_vector));

    let mut index = 0;
    for line in read_to_string(filename).unwrap().lines() {
        index += find_first_number_in_string(line, &num_vector) * 10;
        index += find_last_number_in_string(line, &num_vector);
    }

    println!("The answer is: {index}");
}


fn find_first_number_in_string(source_string: &str, num_vector: &Vec<&str>) -> i32 {
    let mut index = usize::MAX;
    let mut vec_index = 0;

    for (pos, num) in num_vector.iter().enumerate() {
        let optional_location = source_string.find(num);
        
        if !optional_location.is_some(){
            continue;
        }

        let index_in_string = optional_location.unwrap();
        if index_in_string < index {
            index = index_in_string;
            vec_index = pos;
        }
    }

    if vec_index > 10 {
        vec_index -= 10;
    } 

    println!("string: {} \t index: {}", source_string, vec_index);
    return vec_index as i32;

}


fn find_last_number_in_string(source_string: &str, num_vector: &Vec<&str>) -> i32 {
    let mut index = usize::MIN;
    let mut vec_index = 0;

    for (pos, num) in num_vector.iter().enumerate() {
        let optional_location = source_string.find(num);
        
        if !optional_location.is_some(){
            continue;
        }

        let index_in_string = optional_location.unwrap();
        if index_in_string >= index {
            index = index_in_string;
            vec_index = pos;
        }
    }

    if vec_index > 10 {
        vec_index -= 10;
    } 

    println!("string: {} \t index: {}", source_string, vec_index);
    return vec_index as i32;

}

