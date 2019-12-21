fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn exec_int_code(program: &mut [usize]) {
    let mut i: usize = 0;
    while i < program.len() {
        let opcode = program[i];
        match program[i] {
            1 | 2 => {
                let input1 = program[i + 1];
                let input2 = program[i + 2];
                let output = program[i + 3];

                if opcode == 1 {
                    program[output] = program[input1] + program[input2];
                } else {
                    program[output] = program[input1] * program[input2];
                }
            }
            99 => {
                return;
            }
            _ => {
                panic!("Invalid opcode {} at pos {}", opcode, i);
            }
        }
        i += 4;
    }
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
