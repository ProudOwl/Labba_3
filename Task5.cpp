#include <iostream>
#include <vector>
#include <cmath>
#include <iomanip>

using namespace std;

// Функция для моделирования остывания кофе
vector<pair<double, double>> cofe(double Tk, double Tsr, double r, double total_time) {
    vector<pair<double, double>> results;
    const double time_step = 0.1; // Фиксированный интервал 0.1 минуты
    
    for (double t = 0.0; t <= total_time; t += time_step) {
        double T = Tsr + (Tk - Tsr) * exp(-r * t);
        results.emplace_back(t, T);
    }
    return results;
}

int main() {
    // Ввод параметров
    double Tk, Tsr, r, time;
    
    cout << "Введите начальную температуру кофе (Tk): ";
    cin >> Tk;
    cout << "Введите температуру окружающей среды (Tsr): ";
    cin >> Tsr;
    cout << "Введите коэффициент остывания (r): ";
    cin >> r;
    cout << "Введите общее время охлаждения в минутах: ";
    cin >> time;
    
    // Моделирование остывания кофе
    auto cooling_data = cofe(Tk, Tsr, r, time);
    
    // Вывод результатов
    cout << "\nРезультаты моделирования:\n";
    cout << "----------------------------------------\n";
    cout << "Время (мин)\tТемпература (C)\n";
    for (const auto& point : cooling_data) {
        cout << fixed << setprecision(2) << point.first << "\t\t" << point.second << "\n";
    }
    return 0;
}