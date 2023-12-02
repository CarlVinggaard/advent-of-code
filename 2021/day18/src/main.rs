use std::fmt;
use std::fs;
mod parser;

use crate::parser::parse_number;

#[derive(Clone)]
pub enum Number {
    Pair(Box<Number>, Box<Number>),
    Int(u32),
}

#[derive(Clone)]
struct Explosion {
    number: Number,
    right: Option<u32>,
    left: Option<u32>,
    did_explode: bool,
}

#[derive(Clone)]
struct Split {
    number: Number,
    did_split: bool,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            Number::Int(v) => write!(f, "{}", v),
            Number::Pair(l, r) => {
                let left = *l;
                let right = *r;
                write!(f, "[{}, {}]", left, right)
            }
        }
    }
}

fn add(a: Number, b: Number) -> Number {
    let number = Number::Pair(Box::new(a), Box::new(b));

    reduce(number)
}

fn reduce(number: Number) -> Number {
    let mut _number = number;

    while should_explode(&_number, 0) || should_split(&_number) {
        if should_explode(&_number, 0) {
            let explosion = explode(&_number, 0);
            _number = explosion.number;
            continue;
        }

        if should_split(&_number) {
            let split = split(&_number);
            _number = split.number;
        }
    }

    _number
}

fn should_explode(number: &Number, i: usize) -> bool {
    if i > 4 {
        return true;
    }
    match number {
        Number::Int(_) => false,
        Number::Pair(l, r) => should_explode(&*l, i + 1) || should_explode(&*r, i + 1),
    }
}

fn should_split(number: &Number) -> bool {
    match number {
        Number::Int(v) => v > &9,
        Number::Pair(l, r) => should_split(&*l) || should_split(&*r),
    }
}

fn explode(_number: &Number, i: usize) -> Explosion {
    let number = _number.clone();
    match number.clone() {
        Number::Int(_) => Explosion {
            number,
            left: None,
            right: None,
            did_explode: false,
        },
        Number::Pair(l, r) => {
            // If a pair is 4 levels nested, it should explode
            if i >= 4 {
                // EXPLODE
                let left = match *l {
                    Number::Int(v) => v,
                    _ => panic!(),
                };
                let right = match *r {
                    Number::Int(v) => v,
                    _ => panic!(),
                };

                Explosion {
                    number: Number::Int(0),
                    left: Some(left),
                    right: Some(right),
                    did_explode: true,
                }
            } else {
                // Check for explosions below
                // Left
                let l_exp = explode(&*l, i + 1);

                // If left side exploded, add "right" to right side and propagate upwards
                if l_exp.did_explode {
                    let num_r;

                    if let Some(v) = l_exp.right {
                        num_r = add_left(v, *r);
                    } else {
                        num_r = *r;
                    }

                    return Explosion {
                        number: Number::Pair(Box::new(l_exp.number), Box::new(num_r)),
                        right: None,
                        left: l_exp.left,
                        did_explode: true,
                    };
                }

                // Right
                let r_exp = explode(&*r, i + 1);

                // If right side exploded, add "left" to the left side and propagate upwards
                if r_exp.did_explode {
                    let num_l;

                    if let Some(v) = r_exp.left {
                        num_l = add_right(v, l_exp.number);
                    } else {
                        num_l = l_exp.number;
                    }

                    return Explosion {
                        number: Number::Pair(Box::new(num_l), Box::new(r_exp.number)),
                        right: r_exp.right,
                        left: None,
                        did_explode: true,
                    };
                }

                // No side exploded, just propagate upwards
                Explosion {
                    number: Number::Pair(Box::new(l_exp.number), Box::new(r_exp.number)),
                    left: l_exp.left,
                    right: r_exp.right,
                    did_explode: false,
                }
            }
        }
    }
}

fn split(number: &Number) -> Split {
    let _number = number.clone();

    match _number {
        Number::Int(v) => {
            if v > 9 {
                let number = Number::Pair(
                    Box::new(Number::Int((v as f64 / 2.0).floor() as u32)),
                    Box::new(Number::Int((v as f64 / 2.0).ceil() as u32)),
                );
                Split {
                    number,
                    did_split: true,
                }
            } else {
                Split {
                    number: _number,
                    did_split: false,
                }
            }
        }
        Number::Pair(l, r) => {
            // Try to split left
            let l_split = split(&*l);

            if l_split.did_split {
                return Split {
                    number: Number::Pair(Box::new(l_split.number), r),
                    did_split: true,
                };
            }

            let r_split = split(&*r);

            if r_split.did_split {
                return Split {
                    number: Number::Pair(l, Box::new(r_split.number)),
                    did_split: true,
                };
            }

            Split {
                number: number.clone(),
                did_split: false,
            }
        }
    }
}

fn add_left(v: u32, number: Number) -> Number {
    match number {
        Number::Int(i) => Number::Int(v + i),
        Number::Pair(l, r) => {
            let left = add_left(v, *l);
            Number::Pair(Box::new(left), r)
        }
    }
}

fn add_right(v: u32, number: Number) -> Number {
    match number {
        Number::Int(i) => Number::Int(v + i),
        Number::Pair(l, r) => {
            let right = add_right(v, *r);
            Number::Pair(l, Box::new(right))
        }
    }
}

fn calculate_magnitude(number: &Number) -> u32 {
    match number {
        Number::Int(val) => *val,
        Number::Pair(l, r) => 3 * calculate_magnitude(&*l) + 2 * calculate_magnitude(&*r),
    }
}

fn find_largest_magnitude(numbers: Vec<Number>) -> u32 {
    let mut largest = 0;

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            };

            let magnitude = calculate_magnitude(&add(numbers[i].clone(), numbers[j].clone()));

            if magnitude > largest {
                largest = magnitude;
            }
        }
    }

    largest
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("There was a problem reading the file");

    let numbers: Vec<Number> = input.lines().map(|l| parse_number(l)).collect();

    let mut iter = numbers.iter();

    let mut sum: Number = iter.next().unwrap().clone();

    // Add all the numbers in the list one by one
    for number in iter {
        sum = add(sum, number.clone());
    }

    println!("sum: {}", sum);

    let magnitude = calculate_magnitude(&sum);
    println!("magnitude: {}", magnitude);

    // Find the largest magnitude achievable by adding any two numbers in the list
    let largest_magnitude = find_largest_magnitude(numbers);
    println!("largest magnitude: {}", largest_magnitude);
}
