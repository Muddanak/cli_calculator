use std::env;

mod funcs;
use funcs::calculations::*;
use funcs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    let args2: Vec<String> = Vec::from(&args[1..]);

    println!("{:?}, {}", args2, args2.len());
    println!("length of args: {arg_len}");

    {
        if args2.len() >= 3 {
            let mut tmp: Vec<String> = Vec::from(&args2[0..3]);
            let operand = get_operand(&tmp[0], &tmp[1], &tmp[2]);
            tmp.retain(|x| *x != operand.to_string());
            tmp.insert(0, operand.to_string());
            println!("sorted array is: {:?}", tmp);

            match operand {
                '+' => {
                    let outcome = add(
                        Some(tmp[1].parse::<f32>().unwrap_or(0.0)),
                        Some(tmp[2].parse::<f32>().unwrap_or(0.0)),
                    );

                    assert!(outcome.is_ok());

                    println!("Numbers added together: {}", outcome.unwrap());
                }
                '-' => {
                    let outcome = subtract(
                        Some(tmp[1].parse::<f32>().unwrap()),
                        Some(tmp[2].parse::<f32>().unwrap()),
                    );

                    assert!(outcome.is_ok());

                    println!("Numbers subtracted together: {}", outcome.unwrap());
                }
                '*' => {
                    let outcome = multiply(
                        Some(tmp[1].parse::<f32>().unwrap()),
                        Some(tmp[2].parse::<f32>().unwrap()),
                    );

                    assert!(outcome.is_ok());

                    println!("Numbers multiplied together: {}", outcome.unwrap());
                }
                '/' => {
                    let outcome = divide(
                        Some(tmp[1].parse::<f32>().unwrap()),
                        Some(tmp[2].parse::<f32>().unwrap()),
                    );

                    assert!(outcome.is_ok());

                    println!("Numbers divided together: {}", outcome.unwrap());
                }
                'N' => println!("Somehow we got an invalid operand!?"),
                _ => println!("Not yet implemented"),
            }
        }
    }

    println!(
        "Checking if any nums are floats: {}",
        check_for_floats(&args[1], &args[2], &args[3])
    );
    println!(
        "Checking what operand is there: {}",
        get_operand(&args[1], &args[2], &args[3])
    );

    if !check_for_floats(&args[1], &args[2], &args[3]) {
        let sum = add(
            Some(args[1].parse().unwrap()),
            Some(args[3].parse().unwrap()),
        );
        if sum.is_ok() {
            println!("{}", sum.expect("Something happened during the add"));
        }
    }
}
