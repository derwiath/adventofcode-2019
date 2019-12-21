use std::env;
use std::fs;

fn get_fuel_reqs(mass: u32) -> u32 {
    return (mass / 3) - 2;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day1 input-filename");

    println!("Reading mass of modules from {}", filename);

    let module_masses = fs::read_to_string(filename).expect("Failed to read file");

    let mut total_fuel: u32 = 0;
    for line in module_masses.lines() {
        let mass = line.parse::<u32>().unwrap();
        let fuel = get_fuel_reqs(mass);
        total_fuel += fuel;

        println!("{} {}", mass, fuel);
    }

    println!("Total fuel reqs: {}", total_fuel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(get_fuel_reqs(12), 2);
        assert_eq!(get_fuel_reqs(14), 2);
        assert_eq!(get_fuel_reqs(1969), 654);
        assert_eq!(get_fuel_reqs(100756), 33583);
    }
}
