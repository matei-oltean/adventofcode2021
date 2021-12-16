use std::fs;

fn decode_instruction_type(ptr: &mut usize, instructions: &str) -> (usize, usize) {
    let mut next_int = || {
        *ptr += 3;
        usize::from_str_radix(&instructions[*ptr - 3..*ptr], 2).unwrap()
    };
    (next_int(), next_int())
}

fn decode_number(ptr: &mut usize, instructions: &str) -> usize {
    let bytes = instructions.as_bytes();
    let mut res = String::from("");
    let mut end = false;
    while !end {
        end = bytes[*ptr] as char == '0';
        res += &instructions[*ptr + 1..*ptr + 5];
        *ptr += 5;
    }
    usize::from_str_radix(res.as_str(), 2).unwrap()
}

fn do_op(type_id: usize, nums: &[usize]) -> usize {
    match type_id {
        0 => nums.iter().sum(),
        1 => nums.iter().product(),
        2 => *nums.iter().min().unwrap(),
        3 => *nums.iter().max().unwrap(),
        5 => (nums[0] > nums[1]) as usize,
        6 => (nums[0] < nums[1]) as usize,
        _ => (nums[0] == nums[1]) as usize,
    }
}

fn decode_next_instruction(ptr: &mut usize, instructions: &str, versions: &mut usize) -> usize {
    let (version, packet_type) = decode_instruction_type(ptr, instructions);
    *versions += version;
    if packet_type == 4 {
        return decode_number(ptr, instructions);
    }
    let bytes = instructions.as_bytes();
    let type_id = bytes[*ptr];
    let mut acc: Vec<usize> = Vec::new();
    *ptr += 1;
    if type_id as char == '0' {
        let tot_len = usize::from_str_radix(&instructions[*ptr..*ptr + 15], 2).unwrap();
        *ptr += 15;
        let target = *ptr + tot_len;
        while *ptr < target {
            acc.push(decode_next_instruction(ptr, instructions, versions));
        }
    } else {
        let tot_len = usize::from_str_radix(&instructions[*ptr..*ptr + 11], 2).unwrap();
        *ptr += 11;
        acc = (0..tot_len)
            .map(|_| decode_next_instruction(ptr, instructions, versions))
            .collect();
    }
    do_op(packet_type, &acc)
}

fn decode_message(instructions: &str) -> (usize, usize) {
    let mut ptr = 0;
    let mut versions = 0;
    let mut last = 0;
    while ptr < instructions.len() - 7 {
        last = decode_next_instruction(&mut ptr, instructions, &mut versions);
    }
    (versions, last)
}

fn main() {
    let instructions: String = fs::read_to_string("./input/16")
        .unwrap()
        .trim()
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect();
    println!("{:?}", decode_message(instructions.as_str()));
}
