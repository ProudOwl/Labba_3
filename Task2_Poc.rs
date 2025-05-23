use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
use std::fmt;

fn random_generator(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..=max)
}

fn is_prime(i: usize, primes: &[usize]) -> bool {
    primes.iter().all(|&p| i % p != 0)
}

fn eratochene(p: usize) -> Vec<usize> {
    let mut prime_numbers = vec![2];
    let mut i = 3;
    while i < p {
        if is_prime(i, &prime_numbers) {
            prime_numbers.push(i);
        }
        i += 2;
    }
    prime_numbers
}

fn mod_pow(number: u64, power: u64, n: u64) -> u64 {
    let mut result = 1;
    let mut number = number % n;
    let mut power = power;
    while power > 0 {
        if power % 2 == 1 {
            result = (result * number) % n;
        }
        power /= 2;
        number = (number * number) % n;
    }
    result
}

fn ferma(a: usize, x: usize, p: usize) -> bool {
    mod_pow(a as u64, x as u64, p as u64) == 1
}

fn poklington_test(n: u64, factors: &[usize], t: usize) -> bool {
    for _ in 0..t {
        let a = random_generator(2, (n - 1) as usize);
        if !ferma(a, (n - 1) as usize, n as usize) {
            return false;
        }

        let found = factors.iter().any(|&q| {
            ferma(a, ((n - 1) / q as u64) as usize, n as usize)
        });

        if !found {
            return true;
        }
    }
    false
}

fn count_bits(number: u64) -> usize {
    if number == 0 {
        return 1;
    }
    (number as f64).log2().floor() as usize + 1
}

fn is_within_bit_range(number: u64, bit_count: usize, _formin: usize) -> usize {
    let num_bits = count_bits(number);
    if num_bits < bit_count {
        1
    } else if num_bits > bit_count {
        2
    } else {
        3
    }
}

fn create_num(
    k: usize,
    numbers: Arc<Mutex<Vec<u64>>>,
    plusminus: Arc<Mutex<Vec<String>>>,
    oshibki: Arc<Mutex<Vec<usize>>>,
    primes: Arc<Vec<usize>>,
    should_stop: Arc<Mutex<bool>>,
) -> bool {
    let mut q = Vec::new();
    let mut f = 1u64;
    let mut tempf = 1u64;
    let mut temp_oshibki = 0;
    let formin = if k < 8 { 0 } else { 1 };
    let target_bit_count_f = (k / 2) + 1;
    let target_bit_count_r = target_bit_count_f - 1;
    let mut n;

    loop {
        {
            let stop = should_stop.lock().unwrap();
            if *stop {
                return false;
            }
        }

        let qi = primes[random_generator(0, primes.len() / 10)];
        let ai = random_generator(1, 12);
        tempf *= qi.pow(ai as u32) as u64;
        
        match is_within_bit_range(tempf, target_bit_count_f, formin) {
            2 => {
                f = 1;
                tempf = 1;
                q.clear();
            }
            1 => {
                q.push(qi);
            }
            _ => {
                f = tempf;
                q.push(qi);
                let mut r;
                loop {
                    r = random_generator(1, 30) * 2;
                    if is_within_bit_range(r as u64, target_bit_count_r, formin) == 3 {
                        break;
                    }
                }
                n = r as u64 * f + 1;
                if poklington_test(n, &q, 1) {
                    break;
                } else {
                    temp_oshibki += 1;
                    f = 1;
                    tempf = 1;
                    n = 0;
                    q.clear();
                }
            }
        }
    }

    {
        let mut nums = numbers.lock().unwrap();
        let stop = should_stop.lock().unwrap();
        if *stop {
            return false;
        }
        if nums.contains(&n) {
            return false;
        }
    }

    let mut temp_oshibki2 = 0;
    let mut iter = 1;
    let mut i = 0; // Объявляем i здесь
    
    loop {
        {
            let stop = should_stop.lock().unwrap();
            if *stop {
                return false;
            }
        }

        i = 0; // Сбрасываем счетчик перед каждой серией тестов
        temp_oshibki2 = 0; // Сбрасываем счетчик ошибок
        
        for _ in 0..100 {
            if !poklington_test(n, &q, iter) {
                temp_oshibki2 += 1;
            }
            i += 1;
        }

        if temp_oshibki2 <= i / 10 {
            break;
        } else {
            iter += 1;
        }
    }

    {
        let mut nums = numbers.lock().unwrap();
        let mut res = plusminus.lock().unwrap();
        let mut osh = oshibki.lock().unwrap();
        let mut stop = should_stop.lock().unwrap();

        if *stop {
            return false;
        }

        if nums.len() >= 10 {
            return false;
        }

        if temp_oshibki2 <= i / 10 {
            res.push("+".to_string());
        } else {
            res.push("-".to_string());
        }

        nums.push(n);
        osh.push(temp_oshibki);

        if nums.len() >= 10 {
            *stop = true;
        }

        true
    }
}

fn get_single_word_input(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(value) = input.parse::<usize>() {
                if value > 3 && value <= 20 {
                    return value;
                }
            }
        }
        println!("Некорректное число. Попробуйте снова.");
    }
}

struct Results {
    numbers: Vec<u64>,
    results: Vec<String>,
    attempts: Vec<usize>,
    bit_length: usize,
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\nРезультаты генерации простых чисел ({} бит):\n", self.bit_length)?;

        write!(f, "|    Num    |")?;
        for i in 0..self.numbers.len() {
            write!(f, "{:6} |", i + 1)?;
        }

        write!(f, "\n|-----------|")?;
        for _ in 0..self.numbers.len() {
            write!(f, "-------|")?;
        }

        write!(f, "\n|     P     |")?;
        for num in &self.numbers {
            write!(f, "{:6} |", num)?;
        }

        write!(f, "\n| Результат |")?;
        for res in &self.results {
            write!(f, "{:6} |", res)?;
        }

        write!(f, "\n|     K     |")?;
        for att in &self.attempts {
            write!(f, "{:6} |", att)?;
        }

        Ok(())
    }
}

fn main() {
    let numbers = Arc::new(Mutex::new(Vec::new()));
    let results = Arc::new(Mutex::new(Vec::new()));
    let attempts = Arc::new(Mutex::new(Vec::new()));
    let should_stop = Arc::new(Mutex::new(false));

    let bit_length = loop {
        let input = get_single_word_input("Введите размерность простого числа (4-20 бит): ");
        if (4..=20).contains(&input) {
            break input;
        }
        println!("Некорректный ввод. Допустимый диапазон: 4-20 бит.");
    };

    let primes = Arc::new(eratochene(500));
    println!("\nПростые числа до 500 (Решето Эратосфена):\n");
    for (i, prime) in primes.iter().enumerate() {
        print!("{:4}{}", prime, if i % 10 == 9 { "\n" } else { " " });
    }
    println!("\n");

    let num_threads = std::cmp::min(4, std::thread::available_parallelism().unwrap().get());
    let mut handles = vec![];

    for _ in 0..num_threads {
        let numbers = Arc::clone(&numbers);
        let results = Arc::clone(&results);
        let attempts = Arc::clone(&attempts);
        let primes = Arc::clone(&primes);
        let should_stop = Arc::clone(&should_stop);

        handles.push(thread::spawn(move || {
            while !*should_stop.lock().unwrap() {
                create_num(
                    bit_length,
                    numbers.clone(),
                    results.clone(),
                    attempts.clone(),
                    primes.clone(),
                    should_stop.clone(),
                );
            }
        }));
    }

    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(10) {
        {
            let nums = numbers.lock().unwrap();
            let mut stop = should_stop.lock().unwrap();
            if nums.len() >= 10 {
                *stop = true;
                break;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    {
        let mut stop = should_stop.lock().unwrap();
        *stop = true;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_numbers = numbers.lock().unwrap();
    let final_results = results.lock().unwrap();
    let final_attempts = attempts.lock().unwrap();

    let results = Results {
        numbers: final_numbers.clone(),
        results: final_results.clone(),
        attempts: final_attempts.clone(),
        bit_length,
    };

    if !final_numbers.is_empty() {
        println!("{}", results);
    }
}
