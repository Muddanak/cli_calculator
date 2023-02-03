use std::env;

mod funcs;
use crate::funcs::transform::shrink_vector;
use funcs::calculations::*;
use funcs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let arg_len = args.len();
    let mut args2: Vec<String> = Vec::from(&args[1..]);

    //println!("{:?}, {}", args2, args2.len());
    //println!("length of args: {arg_len}");

    {
        let mut outcome: Result<f32, ComputationError> = Ok(0.0);
        for _ in 0..2 {
            if args2.len() >= 3 {
                let mut tmp: Vec<String> = Vec::from(&args2[0..3]);
                let operand = get_operand(&tmp[0], &tmp[1], &tmp[2]);
                tmp.retain(|x| *x != operand.to_string());
                tmp.insert(0, operand.to_string());
                //println!("resorted array is: {tmp:?}");

                match operand {
                    '+' => {
                        outcome = add(
                            Some(tmp[1].parse::<f32>().unwrap_or(0.0)),
                            Some(tmp[2].parse::<f32>().unwrap_or(0.0)),
                        );
                    }
                    '-' => {
                        outcome = subtract(
                            Some(tmp[1].parse::<f32>().unwrap_or(0.0)),
                            Some(tmp[2].parse::<f32>().unwrap_or(0.0)),
                        );
                    }
                    '*' => {
                        outcome = multiply(
                            Some(tmp[1].parse::<f32>().unwrap_or(0.0)),
                            Some(tmp[2].parse::<f32>().unwrap_or(0.0)),
                        );
                    }
                    '/' => {
                        outcome = divide(
                            Some(tmp[1].parse::<f32>().unwrap_or(0.0)),
                            Some(tmp[2].parse::<f32>().unwrap_or(0.0)),
                        );
                    }
                    'N' => println!("Somehow we got an invalid operand!?"),
                    _ => println!("Not yet implemented"),
                }
                //println!("Operand used was {operand} and args len {}", args2.len());
                if args2.len() >= 4 {
                    shrink_vector(&mut args2, *outcome.as_ref().unwrap());
                }
            }
        }
        println!("The result of the computation was {0}", outcome.unwrap())
    }

    /*println!(
        "Checking if any nums are floats: {}",
        check_for_floats(&args[1], &args[2], &args[3])
    );
    println!(
        "Checking what operand is there: {}",
        get_operand(&args[1], &args[2], &args[3])
    );*/

    /*if !check_for_floats(&args[1], &args[2], &args[3]) {
        let sum = add(
            Some(args[1].parse().unwrap_or(0.0)),
            Some(args[3].parse().unwrap_or(0.0)),
        );
        if sum.is_ok() {
            println!("{}", sum.expect("Something happened during the add"));
        }
    }*/
}
