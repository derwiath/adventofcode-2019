use std::env;
use std::fs;

use int_code_machine::run_int_code_program;

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
