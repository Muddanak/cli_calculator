use std::env;

mod funcs;
use crate::funcs::transform::shrink_vector;
use funcs::calculations::*;
use funcs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut args2: Vec<String> = Vec::from(&args[1..]);
    //dbg!(args2.len());
    if args2.len() < 3 {
        println!("Expected at least 3 starting inputs (ex: a + b)");

        return
    }

    {
        let mut outcome: Result<f32, ComputationError> = Ok(0.0);
        while args2.len() > 1 {
            if args2.len() >= 3 {
                let mut tmp: Vec<String> = Vec::from(&args2[0..3]);
                let operand = get_operand(&tmp[0], &tmp[1], &tmp[2]);
                tmp.retain(|x| *x != operand.to_string());
                tmp.insert(0, operand.to_string());

                match operand {
                    '+' => {
                        /*outcome = add(
                            Some(tmp[1].parse::<f32>().unwrap_or(0.0)),
                            Some(tmp[2].parse::<f32>().unwrap_or(0.0)),
                        );*/
                        outcome = g_add(
                            Some(tmp[1].parse::<f32>().unwrap_or(0.0)),
                            Some(tmp[2].parse::<f32>().unwrap_or(0.0)),
                        );
                    }
                    '-' => {
                        outcome = g_subtract(
                            Some(tmp[1].parse::<f32>().unwrap_or(0.0)),
                            Some(tmp[2].parse::<f32>().unwrap_or(0.0)),
                        );
                    }
                    '*' => {
                        outcome = g_multiply(
                            Some(tmp[1].parse::<f32>().unwrap_or(0.0)),
                            Some(tmp[2].parse::<f32>().unwrap_or(0.0)),
                        );
                    }
                    '/' => {
                        outcome = g_divide(
                            Some(tmp[1].parse::<f32>().unwrap_or(0.0)),
                            Some(tmp[2].parse::<f32>().unwrap_or(1.0)),
                        );
                    }
                    'N' => println!("Somehow we got an invalid operand!?"),
                    _ => println!("Not yet implemented"),
                }
                //dbg!(outcome.as_ref());
                if let Ok(result) = outcome.as_ref() {
                    if args2.len() > 2 {
                        //shrink_vector(&mut args2, *outcome.as_ref().unwrap());
                        shrink_vector(&mut args2, *result);
                        //dbg!(&mut args2);
                    }
                } else if let Err(result) = outcome.as_ref() {
                    //dbg!(result);
                    panic!("{}", result.to_string());
                }
            }
        }
        if let Ok(result) = outcome {
            println!("The result of the computation was {result}")
        }
    }
}
