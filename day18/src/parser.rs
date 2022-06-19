use crate::Number;

#[test]
fn test() {
    let vec1: Vec<char> = "[[23,3],[8,9]]".chars().collect();
    assert_eq!(find_middle_index(&vec1), 7);

    let vec2: Vec<char> = "[[[[[9,8],1],2],3],4]".chars().collect();
    assert_eq!(find_middle_index(&vec2), 18);

    let vec3: Vec<char> = "[1,1]".chars().collect();
    assert_eq!(find_middle_index(&vec3), 2);
}

pub fn parse_number(l: &str) -> Number {
    // If its number just return it
    if let Ok(v) = l.parse::<u32>() {
        return Number::Int(v);
    }

    // This is a pair
    let cs: Vec<char> = l.chars().collect();

    // Find the comma in the middle
    let middle_index = find_middle_index(&cs);

    let (left, right) = cs.split_at(middle_index);

    let mut left_str = String::new();
    // Skip one for opening bracket
    for c in left.iter().skip(1) {
        left_str.push(*c);
    }

    let mut right_str = String::new();
    // Skip the last for the closing bracket
    for i in 1..right.len() - 1 {
        right_str.push(right[i]);
    }

    return Number::Pair(
        Box::new(parse_number(left_str.as_str())),
        Box::new(parse_number(right_str.as_str())),
    );
}

fn find_middle_index(cs: &Vec<char>) -> usize {
    let mut iter = cs.iter().skip(1);

    let first = iter.next().unwrap();

    if let Some(_) = first.to_digit(10) {
        // It is a plain number
        for (i, c) in iter.enumerate() {
            if c == &',' {
                return i + 2;
            }
        }
    }

    let mut nested_count = 0;

    for (i, c) in cs.iter().skip(1).enumerate() {
        if c == &'[' {
            nested_count += 1;
        } else if c == &']' {
            nested_count -= 1;
        }

        if nested_count == 0 {
            return i + 2;
        }
    }

    0
}
