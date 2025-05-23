use rand::{thread_rng, Rng};
use std::io::{self, Write};

fn random_generator(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}

fn is_prime(i: usize, primes: &[usize]) -> bool {
    primes.iter().all(|&p| i % p != 0)
}

fn eratosthenes_sieve(p: usize) -> Vec<usize> {
    let mut primes = vec![2];
    for i in (3..p).step_by(2) {
        if is_prime(i, &primes) {
            primes.push(i);
        }
    }
    primes
}

fn mod_exp(mut number: u64, mut power: u64, n: u64) -> u64 {
    let mut remainder = 1;
    while power > 0 {
        if power % 2 != 0 {
            remainder = (remainder * number) % n;
        }
        power /= 2;
        number = (number * number) % n;
    }
    remainder
}

fn fermat_test(a: usize, x: usize, p: usize) -> bool {
    mod_exp(a as u64, x as u64, p as u64) == 1
}

fn miller_test(n: u64, factors: &[usize], t: usize) -> bool {
    for _ in 0..t {
        let a = random_generator(2, (n - 1) as usize);
        if !fermat_test(a, (n - 1) as usize, n as usize) {
            return false;
        }
    }

    for &q in factors {
        let mut all_a_equal_one = true;

        for _ in 0..t {
            let a = random_generator(2, (n - 1) as usize);
            if !fermat_test(a, ((n - 1) / q as u64) as usize, n as usize) {
                all_a_equal_one = false;
                break;
            }
        }

        if all_a_equal_one {
            return false;
        }
    }

    true
}

fn count_bits(number: u64) -> usize {
    if number == 0 {
        1
    } else {
        (number as f64).log2().floor() as usize + 1
    }
}

fn is_within_bit_range(number: u64, bit_count: usize, formin: i32) -> usize {
    let num_bits = count_bits(number);
    if num_bits < bit_count {
        1
    } else if formin >= 0 && num_bits > bit_count - formin as usize {
        2
    } else {
        3
    }
}

fn create_num(
    k: usize,
    numbers: &mut Vec<u64>,
    results: &mut Vec<String>,
    attempts: &mut Vec<usize>,
    primes: &[usize],
) {
    let mut q = Vec::new();
    let mut m: u64 = 1;
    let mut temp_m: u64 = 1;
    let mut n: u64 = 0;
    let target_bit_count_m = k - 1;
    let mut temp_errors = 0;
    let formin = if k < 7 && k > 4 {
        -1
    } else if k == 4 {
        -2
    } else {
        0
    };

    loop {
        let qi = primes[random_generator(0, primes.len() / 10)];
        let ai = random_generator(1, 5); // Уменьшил максимальную степень до 5
        if let Some(new_temp_m) = temp_m.checked_mul((qi as u64).pow(ai as u32)) {
            temp_m = new_temp_m;
        } else {
            m = 1;
            temp_m = 1;
            q.clear();
            continue;
        }
        
        match is_within_bit_range(temp_m, target_bit_count_m, formin) {
            2 => {
                m = 1;
                temp_m = 1;
                q.clear();
            }
            1 => {
                q.push(qi);
            }
            _ => {
                m = temp_m;
                q.push(qi);
                n = 2 * m + 1;
                if miller_test(n, &q, 1) {
                    break;
                } else {
                    temp_errors += 1;
                    m = 1;
                    temp_m = 1;
                    n = 0;
                    q.clear();
                }
            }
        }
    }

    // Проверка на уникальность числа
    while !numbers.is_empty() && numbers.contains(&n) {
        m = 1;
        temp_m = 1;
        q.clear();
        loop {
            let qi = primes[random_generator(0, primes.len() / 10)];
            let ai = random_generator(1, 5);
            if let Some(new_temp_m) = temp_m.checked_mul((qi as u64).pow(ai as u32)) {
                temp_m = new_temp_m;
            } else {
                m = 1;
                temp_m = 1;
                q.clear();
                continue;
            }
            
            match is_within_bit_range(temp_m, target_bit_count_m, formin) {
                2 => {
                    m = 1;
                    temp_m = 1;
                    q.clear();
                }
                1 => {
                    q.push(qi);
                }
                _ => {
                    m = temp_m;
                    q.push(qi);
                    n = 2 * m + 1;
                    if miller_test(n, &q, 1) {
                        break;
                    } else {
                        temp_errors += 1;
                        m = 1;
                        temp_m = 1;
                        n = 0;
                        q.clear();
                    }
                }
            }
        }
    }

    let mut temp_errors2 = 0;
    let mut iter: usize = 100;
    let mut last_iteration: usize = 0;
    
    loop {
        for i in 0..iter {
            if !miller_test(n, &q, 1) {
                temp_errors2 += 1;
            }
            last_iteration = i;
        }
        
        if temp_errors2 <= last_iteration / 10 {
            break;
        } else {
            temp_errors2 = 0;
            iter = iter.saturating_sub(10);
            if iter < 10 {
                iter = 10;
            }
        }
    }

    if temp_errors2 <= last_iteration / 10 {
        results.push("+".to_string());
    } else {
        results.push("-".to_string());
    }
    numbers.push(n);
    attempts.push(temp_errors);
}

fn get_single_word_input(prompt: &str) -> usize {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.split_whitespace().count() != 1 {
            println!("Неправильное количество. Пожалуйста, введите одно число.");
            continue;
        }

        if !input.chars().all(|c| c.is_digit(10)) {
            println!("Некорректное число. Попробуйте снова.");
            continue;
        }

        match input.parse::<usize>() {
            Ok(value) => {
                if value <= 3 || value > 20 {
                    println!("Некорректное число. Попробуйте снова.");
                    continue;
                }
                return value;
            }
            Err(_) => {
                println!("Некорректное число. Попробуйте снова.");
                continue;
            }
        }
    }
}

fn print_primes(primes: &[usize]) {
    println!("Простые числа до 500 (Решето Эратосфена):\n");
    for (i, prime) in primes.iter().enumerate() {
        print!("{:4}", prime);
        if (i + 1) % 10 == 0 {
            println!();
        }
    }
    println!();
}

fn main() {
    let mut numbers = Vec::new();
    let mut results = Vec::new();
    let mut attempts = Vec::new();
    let t = get_single_word_input("Введите размерность простого числа \n");

    let primes = eratosthenes_sieve(500);
    print_primes(&primes);

    for _ in 0..10 {
        create_num(t, &mut numbers, &mut results, &mut attempts, &primes);
    }

    // Вывод результатов
    if !numbers.is_empty() {
        println!("\nРезультаты генерации простых чисел ({} бит):\n", t);
        print!("|    Num    |");
        for i in 0..numbers.len() {
            print!("{:6} |", i + 1);
        }
        println!("\n|-----------|{}", "-------|".repeat(numbers.len()));
        
        print!("|     P     |");
        for num in &numbers {
            print!("{:6} |", num);
        }
        
        println!("\n| Результат |");
        for res in &results {
            print!("{:6} |", res);
        }
        
        println!("\n|     K     |");
        for att in &attempts {
            print!("{:6} |", att);
        }
        println!();
    }
}
