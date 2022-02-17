// Program to print out a pyramid with the number of rows that a user has specified
// The user should be able to specify whether the pyramid should be printed in reverse
// order
/*
    e.g.
    *
    **
    ***
    
    or in reverse:
    ***
    **
    *
*/

fn pyramid(reverse: &str, rows: &i32) {
    if reverse == "reverse-order" {
        for i in 1..rows+1 {
            let mut line = String::new();
            for _n in 0..(rows+1-i) {
                line.push('*');
            }
            println!("{}", line);
        }
    } else if reverse == "in-order" {
        for i in 1..rows+1 {
            let mut line = String::new();
            for _n in 0..i {
                line.push('*');
            }
            println!("{}", line);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let rows: &i32 = &args[1].trim().parse::<i32>().expect("That's not an integer");
    let rev: &str = &args[2][..];

    let valid_values = ["in-order", "reverse-order"];
    if !valid_values.contains(&rev) {
        println!("Invalid order, should be either {} or {}", valid_values[0], valid_values[1]);
        return;
    }
    println!("Printing a pyramid that is {} rows tall", rows);
    println!("it will be {}", rev);
    pyramid(&rev, rows);
}
