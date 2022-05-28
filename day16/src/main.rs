use std::fs;

enum Packet {
    Operator {
        version: u64,
        type_id: u64,
        subpackets: Vec<Packet>,
    },
    Literal {
        version: u64,
        value: u64,
    },
}

fn hex_to_binary(hex: String) -> String {
    let mut out = String::new();

    for c in hex.chars() {
        let b = match c {
            '0' => Some("0000"),
            '1' => Some("0001"),
            '2' => Some("0010"),
            '3' => Some("0011"),
            '4' => Some("0100"),
            '5' => Some("0101"),
            '6' => Some("0110"),
            '7' => Some("0111"),
            '8' => Some("1000"),
            '9' => Some("1001"),
            'A' => Some("1010"),
            'B' => Some("1011"),
            'C' => Some("1100"),
            'D' => Some("1101"),
            'E' => Some("1110"),
            'F' => Some("1111"),
            _ => None,
        };
        if let Some(c) = b {
            out.push_str(c)
        }
    }

    out
}

fn bin_to_decimal(binary: String) -> u64 {
    u64::from_str_radix(binary.as_str(), 2).unwrap()
}

fn parse_literal(binary: String) -> (u64, String) {
    let mut out_str = String::new();

    let mut stop = false;

    let mut chars = binary.chars();

    while !stop {
        let mut chunk_str = String::new();

        for _ in 0..5 {
            chunk_str.push(chars.next().unwrap());
        }

        let mut chunk = chunk_str.chars();

        // Consume the first bit and check if it is a '0'
        if chunk.nth(0) == Some('0') {
            stop = true;
        }

        // Collect digits after the first (which was consumed above)
        let digits: String = chunk.collect();

        out_str.push_str(digits.as_str());
    }

    // Collect remaining
    let rest: String = chars.collect();

    (bin_to_decimal(out_str), rest)
}

fn parse_operator(binary: String) -> (Vec<Packet>, String) {
    let mut chars = binary.chars();

    let first = chars.nth(0).unwrap();

    let mut subpackets = Vec::<Packet>::new();
    let rest: String;

    if first == '0' {
        // If first is '0' the next 15 bits is the number of length in bits of the subpackets
        let subpacket_bit_count = bin_to_decimal(chars.clone().take(15).collect()) as usize;

        let mut subpacket_bits: String = chars.clone().skip(15).take(subpacket_bit_count).collect();

        rest = chars.skip(15 + subpacket_bit_count).collect();

        // Parse the bits into subpackets
        while subpacket_bits.len() > 0 {
            let (packet, _rest) = parse_packet(subpacket_bits.clone());

            subpackets.push(packet);
            subpacket_bits = _rest;
        }
    } else {
        // Else (first == '1'), the next 11 bits is the number of subpackets
        let subpacket_count = bin_to_decimal(chars.clone().take(11).collect()) as usize;

        let mut subpacket_bits: String = chars.clone().skip(11).collect();

        for _ in 0..subpacket_count {
            let (packet, _rest) = parse_packet(subpacket_bits);

            subpackets.push(packet);
            subpacket_bits = _rest;
        }

        rest = subpacket_bits;
    }

    (subpackets, rest)
}

fn parse_packet(binary: String) -> (Packet, String) {
    let binary_chars = binary.chars();

    let version_bin: String = binary_chars.clone().take(3).collect();
    let type_id_bin: String = binary_chars.clone().skip(3).take(3).collect();
    let payload: String = binary_chars.skip(6).collect();

    let version = bin_to_decimal(version_bin);
    let type_id = bin_to_decimal(type_id_bin);

    if type_id == 4 {
        let (value, rest) = parse_literal(payload);

        return (Packet::Literal { version, value }, rest);
    } else {
        let (subpackets, rest) = parse_operator(payload);

        return (
            Packet::Operator {
                version,
                type_id,
                subpackets,
            },
            rest,
        );
    }
}

fn sum_versions(packet: &Packet) -> u64 {
    let mut sum = 0;

    match packet {
        Packet::Literal { version, value: _ } => sum += version,
        Packet::Operator {
            version,
            type_id: _,
            subpackets,
        } => {
            sum += version;

            for packet in subpackets {
                sum += sum_versions(packet);
            }
        }
    }

    sum
}

fn calculate_value(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { version: _, value } => {
            return *value;
        }
        Packet::Operator {
            version: _,
            type_id,
            subpackets,
        } => match type_id {
            &0 => subpackets.iter().map(|p| calculate_value(p)).sum(),
            &1 => subpackets
                .iter()
                .map(|p| calculate_value(p))
                .fold(1, |a, b| a * b),
            &2 => subpackets.iter().map(|p| calculate_value(p)).min().unwrap(),
            &3 => subpackets.iter().map(|p| calculate_value(p)).max().unwrap(),
            &5 => {
                let values: Vec<u64> = subpackets.iter().map(|p| calculate_value(p)).collect();
                if values[0] > values[1] {
                    1
                } else {
                    0
                }
            }
            &6 => {
                let values: Vec<u64> = subpackets.iter().map(|p| calculate_value(p)).collect();
                if values[0] < values[1] {
                    1
                } else {
                    0
                }
            }
            &7 => {
                let values: Vec<u64> = subpackets.iter().map(|p| calculate_value(p)).collect();
                if values[0] == values[1] {
                    1
                } else {
                    0
                }
            }
            _ => {
                println!("This should never happen :/");
                0
            }
        },
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("There was a problem reading the file");

    let binary = hex_to_binary(input);

    let (packet, _rest) = parse_packet(binary);

    let version_sum = sum_versions(&packet);

    println!("Version sum: {}", version_sum);

    let value = calculate_value(&packet);

    println!("Value: {}", value);
}
