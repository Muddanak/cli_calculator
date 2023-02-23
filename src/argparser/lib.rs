
pub(crate) fn parseargs(args: Vec<String>) -> (Vec<i32>, Vec<char>) {
    //let tstr = args.trim().replace(" ", "");
    dbg!(args.clone());
    let mut nvec: Vec<i32> = Vec::new();
    let mut ovec: Vec<char> = Vec::new();
    let out: String = args.join(",");
    dbg!( out );

    for x in args {
        if let Ok(res) = x.parse::<i32>() {
            nvec.push(res);
        } else if let Ok(res) = x.parse::<char>() {
            match res {
                '+' | '-' | '/' | '*' | '(' | ')' => ovec.push(res),
                _ => panic!("Got a wrong operand: {}", res),
            }
        }
    }

    nvec.reverse();
    ovec.reverse();

    assert_eq!(nvec.len() - 1, ovec.len());
    (nvec, ovec)

}
