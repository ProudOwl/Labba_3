use std::io;

fn NOD(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums = input.split_whitespace();
    let a: i32 = nums.next().unwrap().parse().unwrap();
    let b: i32 = nums.next().unwrap().parse().unwrap();

    if b == 1 {
        println!("Infinity");
    }

    let (numerator, denominator) = match a {
        1 => (b, (b - 1).pow(2)),
        2 => (b * (b + 1), (b - 1).pow(3)),
        _ => {
            println!("Irratoinal");
            return;
        }
    };

    let divider = NOD(numerator, denominator);
    let numerator = numerator / divider;
    let denominator = denominator / divider;
    println!("{}/{}", numerator, denominator);
}