fn main() {
    println!("{}", largest_good_integer("999888777666555".to_string()));
}

// 2264. Largest 3-Same-Digit Number in String
pub fn largest_good_integer(num: String) -> String {
    let mut best_int = String::new();
    let digits: Vec<char> = num.chars().collect();
    for i in 0..digits.len() {
        if i + 2 < digits.len() {
            match get_good_int(&digits, i) {
                Some(good_int) => match best_int.cmp(&good_int) {
                    std::cmp::Ordering::Less => best_int = good_int,
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => (),
                },
                None => (),
            }
        } else {
            break;
        }
    }
    return best_int;
}

fn get_good_int(num: &Vec<char>, start: usize) -> Option<String> {
    if num[start] == num[start + 1] && num[start] == num[start + 2] {
        Some(num[start..=start + 2].into_iter().collect())
    } else {
        None
    }
}
