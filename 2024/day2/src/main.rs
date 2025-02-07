use std::fs::read_to_string;

fn main() {
    println!("Day 2, part 1");
    let input = read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut number_of_safe_reports_no_toleration = 0;
    let mut number_of_safe_reports_with_toleration = 0;

    for line in lines {
        let line_list: Vec<&str> = line.split_whitespace().collect();
        let line_list: Vec<i32> = line_list.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        
        if is_safe_report_no_toleration(&line_list) {
            number_of_safe_reports_no_toleration += 1;
        }

        if is_safe_report_with_toleration(line_list) {
            number_of_safe_reports_with_toleration += 1;
        }
    }

    println!("Number of safe reports without toleration: {}", number_of_safe_reports_no_toleration);
    println!("Number of safe reports with toleration: {}", number_of_safe_reports_with_toleration);
}

fn is_safe_report_no_toleration(numbers: &Vec<i32>) -> bool {
    if numbers[1] > numbers[0] {
        for i in 0..numbers.len()-1 {
            if numbers[i + 1] < numbers[i] {
                return false;
            }
            if numbers[i + 1] - numbers[i] > 3 {
                return false;
            }
            if numbers[i] == numbers[i+1] {
                return false;
            }
        }
    }
    else {
        for i in 0..numbers.len()-1 {
            if numbers[i + 1] > numbers[i] {
                return false;
            }
            if numbers[i] - numbers[i+1] > 3 {
                return false;
            }
            if numbers[i] == numbers[i+1] {
                return false;
            }
        }
    }

    return  true;
}

fn is_safe_report_with_toleration(numbers: Vec<i32>) -> bool {
    if is_safe_report_no_toleration(&numbers) {
        return true;
    }

    //5, 4, 3, 2, 3
    for i in 0..numbers.len() {
        let mut templist = numbers.clone();    
        templist.remove(i);

        if is_safe_report_no_toleration(&templist) {
            return true;
        }
    }

    return false;
}



#[test]
fn test_is_safe_function_no_toleration_asc() {
    let test_good_report_asc = vec![1, 2, 3, 4, 5];
    let test_bad_report1_asc = vec![1, 2, 3, 5, 4];
    let test_bad_report2_asc = vec![1, 2, 3, 9, 10];
    let test_bad_report3_asc = vec![1, 2, 3, 3, 4];

    assert_eq!(is_safe_report_no_toleration(&test_good_report_asc), true);
    assert_eq!(is_safe_report_no_toleration(&test_bad_report1_asc), false);
    assert_eq!(is_safe_report_no_toleration(&test_bad_report2_asc), false);
    assert_eq!(is_safe_report_no_toleration(&test_bad_report3_asc), false);
}

#[test]
fn test_is_safe_function_no_toleration_desc() {
    let test_good_report_desc = vec![5, 4, 3, 2, 1];
    let test_bad_report1_desc = vec![5, 4, 3, 2, 3];
    let test_bad_report2_desc = vec![10, 9, 8, 2, 0];
    let test_bad_report3_desc = vec![4, 3, 3, 2, 1];

    assert_eq!(is_safe_report_no_toleration(&test_good_report_desc), true);
    assert_eq!(is_safe_report_no_toleration(&test_bad_report1_desc), false);
    assert_eq!(is_safe_report_no_toleration(&test_bad_report2_desc), false);
    assert_eq!(is_safe_report_no_toleration(&test_bad_report3_desc), false);
}

#[test]
fn test_is_safe_function_with_toleration_asc() {
    let test_good_report_asc = vec![1, 2, 3, 4, 5];
    let test_bad_report1_asc = vec![1, 2, 3, 5, 4];
    let test_bad_report2_asc = vec![1, 2, 3, 9, 10];
    let test_bad_report3_asc = vec![1, 2, 3, 3, 4];
    let test_bad_report4_asc = vec![1, 2, 3, 9, 5];

    assert_eq!(is_safe_report_with_toleration(test_good_report_asc), true);
    assert_eq!(is_safe_report_with_toleration(test_bad_report1_asc), true);
    assert_eq!(is_safe_report_with_toleration(test_bad_report2_asc), false);
    assert_eq!(is_safe_report_with_toleration(test_bad_report3_asc), true);
    assert_eq!(is_safe_report_with_toleration(test_bad_report4_asc), true);
}

#[test]
fn test_is_safe_function_with_toleration_desc() {
    let test_good_report_desc = vec![5, 4, 3, 2, 1];
    let test_bad_report1_desc = vec![5, 4, 3, 2, 3];
    let test_bad_report2_desc = vec![10, 9, 8, 2, 0];
    let test_bad_report3_desc = vec![4, 3, 3, 2, 1];

    assert_eq!(is_safe_report_with_toleration(test_good_report_desc), true);
    assert_eq!(is_safe_report_with_toleration(test_bad_report1_desc ), true);
    assert_eq!(is_safe_report_with_toleration(test_bad_report2_desc), false);
    assert_eq!(is_safe_report_with_toleration(test_bad_report3_desc), true);
}
