use std::env;
use std::fs;
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Exactly one argument is required; the input file");
        process::exit(1);
    }
    let contents = fs::read_to_string(&args[1])?;
    let mut iter: Vec<usize> = contents
        .trim()
        .split(',')
        .map(|x| {
            x.parse::<usize>()
                .expect(&format!("{:} was not a number!", x))
        })
        .collect();

    iter[1] = 12;
    iter[2] = 2;

    let mut pc = 0;
    loop {
        match iter[pc] {
            1 => {
                let i_first = iter[pc + 1];
                let i_second = iter[pc + 2];
                let i_target = iter[pc + 3];
                iter[i_target] = iter[i_first] + iter[i_second];
                println!("Sum! Wrote {:} to pos {:}", iter[i_target], i_target);
            }
            2 => {
                let i_first = iter[pc + 1];
                let i_second = iter[pc + 2];
                let i_target = iter[pc + 3];
                iter[i_target] = iter[i_first] * iter[i_second];
                println!("Multiply! Wrote {:} to pos {:}", iter[i_target], i_target);
            }
            99 => {
                println!("End!");
                break;
            }
            _ => panic!("unexpected op code {:}", iter[pc]),
        }
        pc += 4;
    }

    println!("Result: {:}", iter[0]);

    Ok(())
}
