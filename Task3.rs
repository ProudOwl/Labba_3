use std::io;
use std::cmp;

fn gsd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn pow(base: i32, exponent: i32) -> i32 {
    base.pow(exponent as u32)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    
    let a = parts[0];
    let b = parts[1];
    
    if b == 1 {
        println!("Infinity");
        return;
    }
    
    let (numerator, denominator) = match a {
        1 => (b, pow(b - 1, 2)),
        2 => (b * (b + 1), pow(b - 1, 3)),
        3 => (b * (pow(b, 2) + 4*b + 1), pow(b - 1, 4)),
        4 => (b * (pow(b, 3) + 11*pow(b, 2) + 11*b + 1), pow(b - 1, 5)),
        5 => (b * (pow(b, 4) + 26*pow(b, 3) + 66*pow(b, 2) + 26*b + 1), pow(b - 1, 6)),
        6 => (b * (pow(b, 5) + 57*pow(b, 4) + 302*pow(b, 3) + 302*pow(b, 2) + 57*b + 1), pow(b - 1, 7)),
        7 => (b * (pow(b, 6) + 120*pow(b, 5) + 1191*pow(b, 4) + 2416*pow(b, 3) + 1191*pow(b, 2) + 120*b + 1), pow(b - 1, 8)),
        8 => (b * (pow(b, 7) + 247*pow(b, 6) + 4293*pow(b, 5) + 15619*pow(b, 4) + 15619*pow(b, 3) + 4293*pow(b, 2) + 247*b + 1), pow(b - 1, 9)),
        9 => (b * (pow(b, 8) + 502*pow(b, 7) + 14608*pow(b, 6) + 88234*pow(b, 5) + 156190*pow(b, 4) + 88234*pow(b, 3) + 14608*pow(b, 2) + 502*b + 1), pow(b - 1, 10)),
        10 => (b * (pow(b, 9) + 1013*pow(b, 8) + 47840*pow(b, 7) + 455192*pow(b, 6) + 1310354*pow(b, 5) + 1310354*pow(b, 4) + 455192*pow(b, 3) + 47840*pow(b, 2) + 1013*b + 1), pow(b - 1, 11)),
        _ => {
            println!("Invalid input for a");
            return;
        }
    };
    
    let divider = gsd(numerator, denominator);
    let numerator = numerator / divider;
    let denominator = denominator / divider;
    println!("{}/{}", numerator, denominator);
}
