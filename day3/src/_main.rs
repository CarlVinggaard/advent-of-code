enum Bit {
    Ones,
    Zeroes,
    Equals,
    Eov, // End of vector
}

struct Diagnostic {
    o2: Vec<u32>,
    co2: Vec<u32>,
    gamma: u32,
    epsilon: u32,
}

fn main() {
    let mut diagnostic_: Diagnostic = Diagnostic {
        o2: Vec::new(),
        co2: Vec::new(),
        gamma: 0,
        epsilon: 0,
    };

    parse_report("input.txt", &mut diagnostic_);
    println!(
        "Sub's power consumption: {}",
        diagnostic_.gamma * diagnostic_.epsilon
    );
    println!(
        "Sub's life support rating: {}",
        diagnostic_.o2[0] * diagnostic_.co2[0]
    );
}

fn common_bit(v: &Vec<u32>, pos: u32, bit: u32) -> Vec<u32> {
    let mut ret_vec_: Vec<u32> = Vec::new();

    if v.len() == 1 {
        return v.to_vec();
    }

    for v_ in v {
        if (*v_ >> pos & 1) == bit {
            ret_vec_.push(*v_);
        }
    }

    ret_vec_
}

fn find_common_bit(v: &Vec<u32>, pos: u32) -> Bit {
    let mut zeroes_: u32 = 0;
    let mut ones_: u32 = 0;

    if v.len() <= 1 {
        return Bit::Eov;
    }

    for v_ in v {
        if (*v_ >> pos & 1) == 0 {
            zeroes_ = zeroes_ + 1;
        } else {
            ones_ = ones_ + 1;
        }
    }

    if zeroes_ > ones_ {
        Bit::Zeroes
    } else if ones_ > zeroes_ {
        Bit::Ones
    } else {
        Bit::Equals
    }
}

fn o2_rating(o2: &Vec<u32>) -> Vec<u32> {
    let mut ret_vec_: Vec<u32> = o2.clone();

    for i in (0..12).rev() {
        match find_common_bit(&ret_vec_, i) {
            Bit::Ones => {
                ret_vec_ = common_bit(&ret_vec_, i, 1);
            }
            Bit::Zeroes => {
                ret_vec_ = common_bit(&ret_vec_, i, 0);
            }
            Bit::Equals => {
                ret_vec_ = common_bit(&ret_vec_, i, 1);
            }
            Bit::Eov => {
                break;
            }
        }
    }

    println!("Remaining element (oxygen): {}", ret_vec_[0]);

    ret_vec_
}

fn co2_rating(co2: &Vec<u32>) -> Vec<u32> {
    let mut ret_vec_: Vec<u32> = co2.clone();

    for i in (0..12).rev() {
        match find_common_bit(&ret_vec_, i) {
            Bit::Ones => {
                ret_vec_ = common_bit(&ret_vec_, i, 0);
            }
            Bit::Zeroes => {
                ret_vec_ = common_bit(&ret_vec_, i, 1);
            }
            Bit::Equals => {
                ret_vec_ = common_bit(&ret_vec_, i, 0);
            }
            Bit::Eov => {
                break;
            }
        }
    }

    //println!("Total CO2 scrubber ratings remaining: {}", ret_vec_.len());
    println!("Remaining element (co2): {}", ret_vec_[0]);

    ret_vec_
}

fn gamma_rate(v: &Vec<u32>) -> u32 {
    let mut gamma_: u32 = 0;

    for i in (0..12).rev() {
        match find_common_bit(v, i) {
            Bit::Ones => {
                gamma_ += 1 << i;
            }
            Bit::Zeroes => {
                gamma_ += 0 << i;
            }
            _ => {}
        }
    }

    gamma_
}

fn epsilon_rate(v: &Vec<u32>) -> u32 {
    let mut epsilon_: u32 = 0;

    for i in (0..12).rev() {
        match find_common_bit(v, i) {
            Bit::Ones => {
                epsilon_ += 0 << i;
            }
            Bit::Zeroes => {
                epsilon_ += 1 << i;
            }
            _ => {}
        }
    }

    epsilon_
}

fn parse_report(input: &str, diag: &mut Diagnostic) {
    println!("Parsing report.");
    let mut diag_vec_: Vec<u32> = Vec::new();

    // Read binary codes from input file
    match std::fs::read_to_string(input) {
        Ok(lines) => {
            for line in lines.as_str().lines() {
                diag_vec_.push(u32::from_str_radix(line, 2).unwrap());
            }
        }
        Err(_) => (),
    }

    diag.gamma = gamma_rate(&diag_vec_);
    diag.epsilon = epsilon_rate(&diag_vec_);
    diag.o2 = o2_rating(&diag_vec_);
    diag.co2 = co2_rating(&diag_vec_);
}