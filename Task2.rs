use rand::prelude::*;
use std::io;
use std::io::Write;

fn random_int(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}

fn mod_pow(mut a: u64, mut b: u64, p: u64) -> u64 {
    let mut result = 1;
    a %= p;
    while b > 0 {
        if b % 2 == 1 {
            result = (result * a) % p;
        }
        b >>= 1;
        a = (a * a) % p;
    }
    result
}

fn eratochene(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[p] {
            for i in (p * p..=limit).step_by(p) {
                is_prime[i] = false;
            }
        }
    }
    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &val)| if val { Some(i) } else { None })
        .collect()
}

fn test_ferma(a: u64, x: u64, p: u64) -> bool {
    mod_pow(a, x, p) == 1
}

fn generate_n(k: usize, q: usize, e: usize) -> usize {
    let n = ((2_u64.pow((k - 1) as u32) as f64) / q as f64).ceil()
        + ((2_u64.pow((k - 1) as u32) as f64 * e as f64) / q as f64).ceil();
    let n = n as usize;
    if n % 2 == 0 { n } else { n + 1 }
}

fn diemitko_test(p: u64, t: usize, n: usize, u: usize) -> bool {
    for _ in 0..t {
        if !test_ferma(2, p - 1, p) {
            return false;
        }
        if test_ferma(2, (n + u) as u64, p) {
            return false;
        }
    }
    true
}

fn bit_count(num: u64) -> usize {
    if num == 0 {
        1
    } else {
        64 - num.leading_zeros() as usize
    }
}

fn check_bit_range(num: usize, target_bits: usize, tolerance: usize) -> u8 {
    let bits = bit_count(num as u64);
    if bits < target_bits - tolerance {
        1
    } else if bits > target_bits + tolerance {
        2
    } else {
        3
    }
}

fn generate_prime(
    k: usize,
    primes: &Vec<usize>,
    results: &mut Vec<usize>,
    statuses: &mut Vec<String>,
    attempts: &mut Vec<usize>,
) -> bool {
    let target_q_bits = k / 2;
    let tolerance = if k < 10 {
        if k <= 5 { 2 } else { 1 }
    } else {
        0
    };

    let mut q;
    let mut n;
    let mut u = 0;
    let e = 0;
    let mut p;
    let mut local_attempts = 0;

    loop {
        q = primes[random_int(0, primes.len() - 1)];
        if check_bit_range(q, target_q_bits, tolerance) == 3 {
            n = generate_n(k, q, e);
            p = (n + u) * q + 1;
            if check_bit_range(p, k, tolerance) != 2 && diemitko_test(p as u64, 1, n, u) {
                break;
            }
            u += 2;
            local_attempts += 1;
        }
    }

    while results.contains(&p) {
        u = 0;
        local_attempts = 0;
        loop {
            q = primes[random_int(0, primes.len() - 1)];
            if check_bit_range(q, target_q_bits, tolerance) == 3 {
                n = generate_n(k, q, e);
                p = (n + u) * q + 1;
                if check_bit_range(p, k, tolerance) != 2 && diemitko_test(p as u64, 1, n, u) {
                    break;
                }
                u += 2;
                local_attempts += 1;
            }
        }
    }

    let mut error_count;
    let mut iterations = 1;

    loop {
        error_count = 0;
        for _ in 0..100 {
            if !diemitko_test(p as u64, iterations, n, u) {
                error_count += 1;
            }
        }
        if error_count <= 10 {
            break;
        }
        iterations += 1;
    }

    results.push(p);
    statuses.push(if error_count <= 10 { "+" } else { "-" }.to_string());
    attempts.push(local_attempts);
    true
}

fn get_valid_input(prompt: &str, min_val: usize, max_val: usize) -> usize {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_ok() {
            if let Ok(value) = buffer.trim().parse::<usize>() {
                if value >= min_val && value <= max_val {
                    return value;
                }
            }
        }
        println!("Неверный ввод. Введите число от {} до {}", min_val, max_val);
    }
}

fn main() {
    let t = get_valid_input("Введите размерность простого числа (4-20): ", 4, 20);

    let primes = eratochene(500);
    println!("\nПростые числа до 500:\n");
    for (i, prime) in primes.iter().enumerate() {
        print!("{:3}{}", prime, if i % 10 == 9 { "\n" } else { " " });
    }
    println!();

    let mut results = Vec::new();
    let mut statuses = Vec::new();
    let mut attempts = Vec::new();

    for _ in 0..10 {
        generate_prime(t, &primes, &mut results, &mut statuses, &mut attempts);
    }

    println!("\nПростые числа, созданные с помощью ГОСТ:\n");

    print!("|    Num    |");
    for i in 1..=10 {
        print!("{:6} |", i);
    }
    println!();

    print!("\n|     P     |");
    for p in &results {
        print!("{:6} |", p);
    }
    println!();

    print!("\n| Результат |");
    for s in &statuses {
        print!("{:6} |", s);
    }
    println!();

    print!("\n|     K     |");
    for a in &attempts {
        print!("{:6} |", a);
    }
    println!();
}
