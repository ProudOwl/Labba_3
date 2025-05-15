use std::io;

// Функция для моделирования остывания кофе
fn cofe(tk: f64, tsr: f64, r: f64, total_time: f64) -> Vec<(f64, f64)> {
    let mut results = Vec::new();
    let time_step = 0.1; // Фиксированный интервал 0.1 минуты
    
    let mut t = 0.0;
    while t <= total_time {
        let temp = tsr + (tk - tsr) * (-r * t).exp();
        results.push((t, temp));
        t += time_step;
    }
    
    results
}

fn main() {
    // Ввод параметров
    println!("Введите начальную температуру кофе (Tk): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let tk: f64 = input.trim().parse().unwrap();

    println!("Введите температуру окружающей среды (Tsr): ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let tsr: f64 = input.trim().parse().unwrap();

    println!("Введите коэффициент остывания (r): ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let r: f64 = input.trim().parse().unwrap();

    println!("Введите общее время охлаждения в минутах: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let time: f64 = input.trim().parse().unwrap();
    
    // Моделирование остывания кофе
    let cooling_data = cofe(tk, tsr, r, time);
    
    // Вывод результатов
    println!("\nРезультаты моделирования:");
    println!("----------------------------------------");
    println!("Время (мин)\tТемпература (C)");
    for point in cooling_data {
        println!("{:.2}\t\t{:.2}", point.0, point.1);
    }
}