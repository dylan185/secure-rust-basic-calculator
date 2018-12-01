/*
 * Dylan Forsyth 
 * Secure Programming Assignment #2
 * October 4th 2018
 */

use std::io;

// fn add(x: Option<i32>, y: Option<i32>) -> Option<i32>, Option<i32>{

// }

fn main() {
    // Start main loop, exit if res has a number
    let mut done = false;
    while !done{
        let mut op = 'c';
        let mut inop = String::new();
        let mut name = String::new();
        let mut inx = String::new();
        let mut iny = String::new();
        let mut res = 0;
        println!("Please Enter Name: ");
        io::stdin().read_line(&mut name).expect("Invalid Name.");
        println!("Please Enter First Number: ");
        io::stdin().read_line(&mut inx).expect("Invalid Number.");
        let x = inx.trim().parse::<i32>().unwrap();
        println!("Please Enter Second Number: ");
        io::stdin().read_line(&mut iny).expect("Invalid Number.");
        let y = iny.trim().parse::<i32>().unwrap();
        println!("Please Enter Operator[a(+), s(-), m(*), d(/), c(close)]: ");
        io::stdin().read_line(&mut inop).expect("Invalid Choice.");
        op = inop.chars().next().expect("Enter a choice.");
        
        match op {
            'a' => {
                res = x + y;
                println!("Thanks {}, {} + {} = {}", name, x, y, res);
            },
            's' => {
                res = x - y;
                println!("Thanks {}, {} - {} = {}", name, x, y, res);
            },
            'm' => {
                res = x * y;
                println!("Thanks {}, {} * {} = {}", name, x, y, res);
            },
            'd' => {
                res = x/y;
                println!("Thanks {}, {} / {} = {}", name, x, y, res);
            },
            'c' => {
                done=true;
            },
            _ => { /* ignore everything else */ }
        }
    }
}
