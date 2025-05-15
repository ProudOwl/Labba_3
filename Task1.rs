fn function(x: f64) -> f64 {
    if (-8.0 <= x) && (x < -5.0) {
        -3.0
    } else if (-5.0 <= x) && (x < -3.0) {
        (3.0 * x + 9.0) / 2.0
    } else if (-3.0 <= x) && (x < 3.0) {
        (9.0 - x.powi(2)).sqrt()
    } else if (3.0 <= x) && (x < 8.0) {
        (3.0 * x - 9.0) / 5.0
    } else if (8.0 <= x) && (x <= 10.0) {
        3.0
    }
}

fn main() {
    let x_start = -8.0;  // Начальное значение
    let x_end = 10.0;    // Конечное значение
    let dx = 0.5;        // Шаг

    println!(" x\t y");
    println!("-----------");
    
    let mut x = x_start;
    while x <= x_end {
        let y = function(x);
        
        // Вывод результата на экран
        println!("{:.1}\t{:.4}", x, y);
        
        x += dx;
    }
}