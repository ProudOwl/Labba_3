use std::io;
use std::collections::HashSet;
use rand::Rng;

// Решето Эратосфена для нахождения всех простых чисел до n
fn eratosthene(n: usize) -> Vec<usize> {
    let mut prime = vec![true; n + 1];
    prime[0] = false;
    prime[1] = false;
    
    let mut p = 2;
    while p * p <= n {
        if prime[p] {
            let mut i = p * p;
            while i <= n {
                prime[i] = false;
                i += p;
            }
        }
        p += 1;
    }
    
    prime.iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(i, _)| i)
        .collect()
}

// Функция для возведения в степень по модулю (a^b mod m)
fn mod_pow(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut result = 1;
    a %= m;
    while b > 0 {
        if b % 2 == 1 {
            result = (result * a) % m;
        }
        a = (a * a) % m;
        b /= 2;
    }
    result
}

// Тест для проверки простоты числа
fn test(n: u64, k: usize) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // Разложение n-1 в виде d * 2^s
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let mut rng = rand::thread_rng();

    for _ in 0..k {
        let a = rng.gen_range(2..n-1);
        let mut x = mod_pow(a, d, n);

        if x == 1 || x == n - 1 {
            continue;
        }

        let mut composite = true;
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

// ГОСТ Р 34.10-94
fn gost(t: usize, q: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let lower = 1u64 << (t - 1);
    let upper = (1u64 << t) - 1;

    loop {
        let mut n = rng.gen_range(lower..=upper);
        if n % 2 == 0 {
            n += 1;
        }

        // Проверяем, что N = q * k + 1 для некоторого k
        if (n - 1) % q != 0 {
            continue;
        }

        // Проверяем N на простоту
        if test(n, 5) { // 5 итераций для надёжности
            return n;
        }
    }
}

fn main() {
    // 1. Решето Эратосфена
    let primes = eratosthene(500);
    println!("Простые числа < 500:");
    for (i, &prime) in primes.iter().enumerate() {
        print!("{}{}", prime, if i % 10 == 9 { "\n" } else { " " });
    }
    println!("\n");

    // Ввод пользователем значений t и q (10 пар)
    let mut t = vec![0; 10];
    let mut q = vec![0; 10];
    println!("Введите 10 пар значений (t q), где:");
    println!("- t - размерность простого числа в битах");
    println!("- q - простое число (из таблицы выше)");

    let prime_set: HashSet<usize> = primes.iter().cloned().collect();

    for i in 0..10 {
        println!("Пара {}: ", i + 1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        t[i] = parts[0].parse().unwrap();
        q[i] = parts[1].parse().unwrap();
        
        // Проверка, что q является простым числом из таблицы
        if !prime_set.contains(&(q[i] as usize)) {
            eprintln!("Ошибка: q должно быть простым числом из таблицы");
            std::process::exit(1);
        }
    }
    println!();

    // Генерация 10 простых чисел на основе введённых t и q
    let mut generated_primes = Vec::new();
    
    for i in 0..10 {
        let p = gost(t[i], q[i] as u64);
        generated_primes.push(p);
    }
    
    // Проверка вероятностным тестом
    let iterations = 2; // Количество итераций для вероятности ошибки < 0.1
    
    let mut test_results = Vec::new();
    for &p in &generated_primes {
        let is_prime = test(p, iterations);
        test_results.push(is_prime);
    }
    
    // 5. Проверка отвергнутых чисел
    let mut k = 0;
    for i in 0..generated_primes.len() {
        if !test_results[i] {
            // Повторная проверка с увеличенным количеством итераций
            let is_prime = test(generated_primes[i], iterations * 2);
            if is_prime {
                k += 1;
            }
        }
    }
    
    println!("Результаты:");
    println!("+----+---------------------+----------------------------------+-----+");
    println!("| №  | P                   | Результат проверки вероятностным | K   |");
    println!("|    |                     | тестом                           |     |");
    println!("+----+---------------------+----------------------------------+-----+");
    
    for i in 0..10 {
        println!("| {:2} | {:19} | {:32} | {:3} |", 
            i + 1, 
            generated_primes[i], 
            if test_results[i] { "+" } else { "-" }, 
            if i == 0 { k } else { 0 });
    }
    
    println!("+----+---------------------+----------------------------------+-----+");
}