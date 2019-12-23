use std::env;
use std::fs;

enum Instruction {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
}

fn read_instruction(memory: &[usize], ip: usize) -> Instruction {
    match memory[ip] {
        1 => Instruction::Add(memory[ip + 1], memory[ip + 2], memory[ip + 3]),
        2 => Instruction::Mul(memory[ip + 1], memory[ip + 2], memory[ip + 3]),
        99 => Instruction::Halt,
        _ => panic!("Invalid opcode {} at {}", memory[0], ip),
    }
}

fn exec_int_code(memory: &mut [usize]) {
    let mut ip: usize = 0;
    while ip < memory.len() {
        let instr = read_instruction(&memory[..], ip);

        match instr {
            Instruction::Add(input1, input2, output) => {
                memory[output] = memory[input1] + memory[input2];
                ip += 4;
            }
            Instruction::Mul(input1, input2, output) => {
                memory[output] = memory[input1] * memory[input2];
                ip += 4;
            }
            Instruction::Halt => return,
        }
    }
}

fn run_int_code_program(initial_state: &[usize], noun: usize, verb: usize) -> usize {
    let mut program = initial_state.to_vec();
    program[1] = noun;
    program[2] = verb;
    exec_int_code(&mut program[..]);
    program[0]
}

fn find_noun_and_verb_for_day2_part2(initial_state: &[usize]) {
    const DESIRED_ANSWER: usize = 19690720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let value = run_int_code_program(&initial_state, noun, verb);
            if value == DESIRED_ANSWER {
                println!("noun={} and verb={} yields: {}", noun, verb, value);
                return;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day2 input-filename");

    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let initial_state: Vec<usize> = contents
        .trim_end()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|token| token.parse::<usize>().unwrap())
        .collect();

    let noun = 12;
    let verb = 2;
    println!(
        "noun={} and verb={} yields: {}",
        noun,
        verb,
        run_int_code_program(&initial_state, noun, verb)
    );

    find_noun_and_verb_for_day2_part2(&initial_state[..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut input: [usize; 12] = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let output: [usize; 12] = [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        exec_int_code(&mut input[..]);
        assert_eq!(input, output);
    }

    #[test]
    fn test_2() {
        let mut input: [usize; 5] = [1, 0, 0, 0, 99];
        let output: [usize; 5] = [2, 0, 0, 0, 99];
        exec_int_code(&mut input[..]);
        assert_eq!(input, output);
    }

    #[test]
    fn test_3() {
        let mut input: [usize; 5] = [2, 3, 0, 3, 99];
        let output: [usize; 5] = [2, 3, 0, 6, 99];
        exec_int_code(&mut input[..]);
        assert_eq!(input, output);
    }

    #[test]
    fn test_4() {
        let mut input: [usize; 6] = [2, 4, 4, 5, 99, 0];
        let output: [usize; 6] = [2, 4, 4, 5, 99, 9801];
        exec_int_code(&mut input[..]);
        assert_eq!(input, output);
    }

    #[test]
    fn test_5() {
        let mut input: [usize; 9] = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output: [usize; 9] = [30, 1, 1, 4, 2, 5, 6, 0, 99];
        exec_int_code(&mut input[..]);
        assert_eq!(input, output);
    }

    #[test]
    fn test_day2_part2() {
        let initial_state = vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 9, 19, 1, 19, 5, 23, 2, 6, 23,
            27, 1, 6, 27, 31, 2, 31, 9, 35, 1, 35, 6, 39, 1, 10, 39, 43, 2, 9, 43, 47, 1, 5, 47,
            51, 2, 51, 6, 55, 1, 5, 55, 59, 2, 13, 59, 63, 1, 63, 5, 67, 2, 67, 13, 71, 1, 71, 9,
            75, 1, 75, 6, 79, 2, 79, 6, 83, 1, 83, 5, 87, 2, 87, 9, 91, 2, 9, 91, 95, 1, 5, 95, 99,
            2, 99, 13, 103, 1, 103, 5, 107, 1, 2, 107, 111, 1, 111, 5, 0, 99, 2, 14, 0, 0,
        ];

        assert_eq!(run_int_code_program(&initial_state[..], 80, 18), 19690720);
    }
}
