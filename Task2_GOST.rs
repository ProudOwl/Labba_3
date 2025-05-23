use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
use std::cmp::min;
use std::f64::consts::LN_2;
use std::collections::HashSet;

static STOP_FLAG: AtomicBool = AtomicBool::new(false);

fn random_int(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..=max)
}

fn mod_pow(mut a: u64, mut b: u64, n: u64) -> u64 {
    let mut result = 1;
    a %= n;
    
    while b > 0 {
        if b % 2 == 1 {
            result = (result * a) % n;
        }
        b >>= 1;
        a = (a * a) % n;
    }
    result
}

fn eratosthenes_sieve(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for p in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[p] {
            for i in (p*p..=limit).step_by(p) {
                is_prime[i] = false;
            }
        }
    }
    
    is_prime.iter()
        .enumerate()
        .filter(|&(_, &is_p)| is_p)
        .map(|(i, _)| i)
        .collect()
}

fn is_prime_simple(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn diemitko_test(n: u64, n_param: u64, u: u64) -> bool {
    // Проверка 1: 2^(N+u) mod n != 1
    if mod_pow(2, n_param + u, n) == 1 {
        return false;
    }
    
    // Проверка 2: 2^(n-1) mod n == 1
    mod_pow(2, n - 1, n) == 1
}

fn is_in_bit_range(number: usize, target_bits: usize, tolerance: usize) -> bool {
    if number == 0 {
        return false;
    }
    let actual_bits = (number as f64).log2().floor() as usize + 1;
    actual_bits >= target_bits.saturating_sub(tolerance) && 
    actual_bits <= target_bits + tolerance
}

fn generate_prime(
    k: usize,
    primes: &[usize],
    numbers: Arc<Mutex<Vec<usize>>>,
    results: Arc<Mutex<Vec<String>>>,
    attempts: Arc<Mutex<Vec<usize>>>,
) -> bool {
    let q_bits = k / 2;
    let tolerance = if k < 10 {
        if k <= 5 { 2 } else { 1 }
    } else {
        0
    };
    
    let upper_bound = (1u64 << k) - 1;
    let lower_bound = 1u64 << (k - 1);
    
    let max_attempts = 1000;
    let mut attempt_count = 0;
    
    while !STOP_FLAG.load(Ordering::Relaxed) && attempt_count < max_attempts {
        let q = primes[random_int(0, primes.len() - 1)];
        if !is_in_bit_range(q, q_bits, tolerance) {
            continue;
        }
        
        // Вычисляем N
        let mut n_param = ((1 << (k-1)) / q);
        if n_param % 2 != 0 {
            n_param += 1;
        }
        
        let mut u = 0;
        loop {
            let p = (n_param + u) * q + 1;
            
            if p > upper_bound as usize {
                break;
            }
            
            if p >= lower_bound as usize && p <= upper_bound as usize {
                if is_prime_simple(p) && diemitko_test(p as u64, n_param as u64, u as u64) {
                    let mut numbers_lock = numbers.lock().unwrap();
                    let mut results_lock = results.lock().unwrap();
                    let mut attempts_lock = attempts.lock().unwrap();
                    
                    if !numbers_lock.contains(&p) {
                        numbers_lock.push(p);
                        results_lock.push("+".to_string());
                        attempts_lock.push(u);
                        return true;
                    }
                }
            }
            u += 2;
            attempt_count += 1;
            
            if STOP_FLAG.load(Ordering::Relaxed) || attempt_count >= max_attempts {
                break;
            }
        }
    }
    false
}

fn valid_input(prompt: &str, min_val: usize, max_val: usize) -> usize {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<usize>() {
            Ok(value) if value >= min_val && value <= max_val => return value,
            _ => println!("Ошибка. Введите число от {} до {}", min_val, max_val),
        }
    }
}

fn main() {
    let t = valid_input("Введите размерность простого числа (4-19): ", 4, 19);
    
    let primes = eratosthenes_sieve(500);
    println!("\nПростые числа до 500 (Решето Эратосфена):\n");
    for (i, prime) in primes.iter().enumerate() {
        print!("{:4}{}", prime, if i % 10 == 9 { "\n" } else { " " });
    }
    println!("\n");
    
    let numbers = Arc::new(Mutex::new(Vec::new()));
    let results = Arc::new(Mutex::new(Vec::new()));
    let attempts = Arc::new(Mutex::new(Vec::new()));
    
    let mut handles = vec![];
    let num_threads = min(4, num_cpus::get());
    
    for _ in 0..num_threads {
        let numbers_clone = Arc::clone(&numbers);
        let results_clone = Arc::clone(&results);
        let attempts_clone = Arc::clone(&attempts);
        let primes_clone = primes.clone();
        
        handles.push(thread::spawn(move || {
            while !STOP_FLAG.load(Ordering::Relaxed) && numbers_clone.lock().unwrap().len() < 10 {
                generate_prime(
                    t,
                    &primes_clone,
                    Arc::clone(&numbers_clone),
                    Arc::clone(&results_clone),
                    Arc::clone(&attempts_clone),
                );
            }
        }));
    }
    
    let start = Instant::now();
    while Instant::now() - start < Duration::from_secs(10) {
        if numbers.lock().unwrap().len() >= 10 {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    STOP_FLAG.store(true, Ordering::Relaxed);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let numbers_lock = numbers.lock().unwrap();
    let results_lock = results.lock().unwrap();
    let attempts_lock = attempts.lock().unwrap();
    
    println!("\nРезультаты генерации простых чисел ({} бит):\n", t);
    
    print!("|    Num    |");
    for i in 0..numbers_lock.len() {
        print!("{:6} |", i + 1);
    }
    
    print!("\n|-----------|");
    for _ in 0..numbers_lock.len() {
        print!("-------|");
    }
    
    print!("\n|     P     |");
    for num in numbers_lock.iter() {
        print!("{:6} |", num);
    }
    
    print!("\n| Результат |");
    for res in results_lock.iter() {
        print!("{:6} |", res);
    }
    
    print!("\n|     K     |");
    for att in attempts_lock.iter() {
        print!("{:6} |", att);
    }
    println!();
}
