use std::env;
use std::fs;

fn get_fuel_reqs(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn get_fuel_reqs_incl_fuel(mass: i32) -> i32 {
    let mut fuel = get_fuel_reqs(mass);
    let mut total_fuel = fuel;
    while fuel > 0 {
        fuel = get_fuel_reqs(fuel);
        if fuel > 0 {
            total_fuel += fuel;
        } else {
            break;
        }
    }
    total_fuel
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day1 input-filename");

    println!("Reading mass of modules from {}", filename);

    let module_masses = fs::read_to_string(filename).expect("Failed to read file");

    let mut total_fuel: i32 = 0;
    for line in module_masses.lines() {
        let mass = line.parse::<i32>().unwrap();
        let fuel = get_fuel_reqs_incl_fuel(mass);
        total_fuel += fuel;

        println!("{} {}", mass, fuel);
    }

    println!("Total fuel reqs: {}", total_fuel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_reqs() {
        assert_eq!(get_fuel_reqs(12), 2);
        assert_eq!(get_fuel_reqs(14), 2);
        assert_eq!(get_fuel_reqs(1969), 654);
        assert_eq!(get_fuel_reqs(100756), 33583);
    }

    #[test]
    fn test_fuel_reqs_incl_fuel() {
        assert_eq!(get_fuel_reqs_incl_fuel(12), 2);
        assert_eq!(get_fuel_reqs_incl_fuel(14), 2);
        assert_eq!(get_fuel_reqs_incl_fuel(1969), 966);
        assert_eq!(get_fuel_reqs_incl_fuel(100756), 50346);
    }
}
