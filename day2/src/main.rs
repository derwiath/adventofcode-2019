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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day2 input-filename");

    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let mut program: Vec<usize> = contents
        .trim_end()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|token| token.parse::<usize>().unwrap())
        .collect();

    program[1] = 12;
    program[2] = 2;

    exec_int_code(&mut program[..]);
    println!("Value at position 0: {}", program[0]);
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
}
