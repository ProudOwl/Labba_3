use std::io;

// Реализация gcd
fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Функция для проверки, является ли число рациональным
fn is_rational(sum: f64, max_denominator: i64) -> bool {
    for denom in 1..=max_denominator {
        let numerator = (sum * denom as f64).round();
        if (numerator / denom as f64 - sum).abs() < 1e-9 {
            return true;
        }
    }
    false
}

// Функция для представления числа в виде дроби
fn to_fraction(sum: f64, max_denominator: i64) -> (i64, i64) {
    for denominator in 1..=max_denominator {
        let numerator = (sum * denominator as f64).round() as i64;
        if (numerator as f64 / denominator as f64 - sum).abs() < 1e-9 {
            let common_divisor = gcd(numerator.abs(), denominator);
            return (numerator / common_divisor, denominator / common_divisor);
        }
    }
    (sum.round() as i64, 1)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Проверка на пустой ввод
    if input.trim().is_empty() {
        println!("Please enter two numbers separated by space");
        return;
    }
    
    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Проверка, что введено ровно два числа
    if nums.len() != 2 {
        println!("Please enter exactly two numbers separated by space");
        return;
    }

    let (a, b) = (nums[0], nums[1]);

    // Проверка сходимости по признаку Даламбера
    if b <= 1 {
        println!("infinity");
        return;
    }

    // Численное приближение суммы ряда с защитой от переполнения
    let mut sum = 0.0;
    let epsilon = 1e-10;
    let mut term;
    let mut n: i32 = 1;
    loop {
        let numerator = match n.checked_pow(a as u32) {
            Some(val) => val as f64,
            None => {
                break;
            }
        };
        
        let denominator = match b.checked_pow(n as u32) {
            Some(val) => val as f64,
            None => {
                break;
            }
        };
        
        term = numerator / denominator;
        sum += term;
        n += 1;
        
        if term <= epsilon {
            break;
        }
    }

    // Проверка на рациональность
    if is_rational(sum, 1000) {
        let (numerator, denominator) = to_fraction(sum, 1000);
        println!("{}/{}", numerator, denominator);
    } else {
        println!("irrational");
    }
}
