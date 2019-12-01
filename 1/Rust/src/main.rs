use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Exactly one argument is required; the input file");
        process::exit(1);
    }
    let filename = &args[1];
    let file = fs::File::open(filename).unwrap();
    let mut buf_reader = io::BufReader::new(file);

    let mut total_fuel: i64 = 0;
    loop {
        let mut line = String::new();
        let len = buf_reader.read_line(&mut line).unwrap();
        if len == 0 {
            println!("Total fuel: {:}", total_fuel);
            return;
        }
        let module_mass = line.trim().parse::<i64>().unwrap();
        total_fuel += get_fuel_requirements(module_mass);
    }
}

fn get_fuel_requirements(mass: i64) -> i64 {
    let fuel_requirements = (mass / 3) - 2;
    if fuel_requirements <= 0 {
        return 0;
    }
    return fuel_requirements + get_fuel_requirements(fuel_requirements);
}
