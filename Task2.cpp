#include <iostream>
#include <vector>
#include <iomanip>
#include <random>
#include <cmath>
#include <algorithm>
using namespace std;

// генерация случайного числа
int random_int(int min, int max) {
    static mt19937 gen(random_device{}());
    uniform_int_distribution<int> dist(min, max);
    return dist(gen);
}

// модульное возведение в степень
uint64_t mod_pow(uint64_t a, uint64_t b, uint64_t p) {
    uint64_t result = 1;
    a %= p;

    while (b > 0) {
        if (b % 2 == 1)
            result = (result * a) % p;
        b >>= 1;
        a = (a * a) % p;
    }
    return result;
}

// решето Эратосфена
vector<int> eratochene(int limit) {
    vector<bool> is_prime(limit + 1, true);
    is_prime[0] = is_prime[1] = false;

    for (int p = 2; p * p <= limit; ++p) {
        if (is_prime[p]) {
            for (int i = p * p; i <= limit; i += p)
                is_prime[i] = false;
        }
    }

    vector<int> primes;
    for (int i = 2; i <= limit; ++i) {
        if (is_prime[i]) primes.push_back(i);
    }
    return primes;
}

bool test_ferma(int a, int x, int p) {
    return mod_pow(a, x, p) == 1;
}

int generate_N(int k, int q, int e) {
    int N = ceil(pow(2, k - 1) / q) + ceil((pow(2, k - 1) * e) / q);
    return (N % 2 == 0) ? N : N + 1;
}

// тест Диемитко
bool diemitko_test(int64_t p, int t, int N, int u) {
    for (int i = 0; i < t; ++i) {
        if (!test_ferma(2, p - 1, p)) return false;
        if (test_ferma(2, N + u, p)) return false;
    }
    return true;
}

// проверка количества бит в числе
int bit_count(uint64_t num) {
    return num == 0 ? 1 : static_cast<int>(log2(num)) + 1;
}

// проверка соответствия числа заданному диапазону бит
int check_bit_range(int num, int target_bits, int tolerance) {
    int bits = bit_count(num);
    if (bits < target_bits - tolerance) return 1;
    if (bits > target_bits + tolerance) return 2;
    return 3;
}

// функция генерации простого числа
bool generate_prime(int k, vector<int>& primes, vector<int>& results, vector<string>& statuses, vector<int>& attempts) {
    const int target_q_bits = k / 2;
    int tolerance = (k < 10) ? (k <= 5 ? 2 : 1) : 0;

    int q, N, u = 0, e = 0, p;
    int local_attempts = 0;

    while (true) {
        q = primes[random_int(0, primes.size() - 1)];

        if (check_bit_range(q, target_q_bits, tolerance) == 3) {
            N = generate_N(k, q, e);
            p = (N + u) * q + 1;

            if (check_bit_range(p, k, tolerance) != 2 && diemitko_test(p, 1, N, u)) {
                break;
            }
            u += 2;
            local_attempts++;
        }
    }

    // проверка на уникальность
    while (find(results.begin(), results.end(), p) != results.end()) {
        u = 0;
        local_attempts = 0;

        while (true) {
            q = primes[random_int(0, primes.size() - 1)];

            if (check_bit_range(q, target_q_bits, tolerance) == 3) {
                N = generate_N(k, q, e);
                p = (N + u) * q + 1;

                if (check_bit_range(p, k, tolerance) != 2 && diemitko_test(p, 1, N, u)) {
                    break;
                }
                u += 2;
                local_attempts++;
            }
        }
    }

    // тестирование надежности
    int error_count = 0;
    int iterations = 1;

    while (true) {
        error_count = 0;
        for (int i = 0; i < 100; ++i) {
            if (!diemitko_test(p, iterations, N, u)) error_count++;
        }

        if (error_count <= 10) break;
        iterations++;
    }

    results.push_back(p);
    statuses.push_back(error_count <= 10 ? "+" : "-");
    attempts.push_back(local_attempts);

    return true;
}

// Функция для безопасного ввода числа
int get_valid_input(const string& prompt, int min_val, int max_val) {
    int value;
    while (true) {
        cout << prompt;
        if (cin >> value && value >= min_val && value <= max_val) {
            cin.ignore(numeric_limits<streamsize>::max(), '\n');
            return value;
        }
        cin.clear();
        cin.ignore(numeric_limits<streamsize>::max(), '\n');
        cout << "Неверный ввод. Введите число от " << min_val << " до " << max_val << "\n";
    }
}

int main() {
    int t = get_valid_input("Введите размерность простого числа (4-20): ", 4, 20);

    vector<int> primes = eratochene(500);
    cout << "\nПростые числа до 500:\n";
    for (size_t i = 0; i < primes.size(); ++i) {
        cout << primes[i] << (i % 10 == 9 ? "\n" : " ");
    }
    cout << "\n\n";

    vector<int> results;
    vector<string> statuses;
    vector<int> attempts;

    // Генерация 10 простых чисел
    for (int i = 0; i < 10; ++i) {
        generate_prime(t, primes, results, statuses, attempts);
    }

    // Вывод результатов
    cout << "Простые числа, созданные с помощью ГОСТ:\n\n";

    cout << "|    Num    |";
    for (int i = 1; i <= 10; ++i) cout << setw(6) << i << " |";

    cout << "\n\n|     P     |";
    for (int i = 0; i < 10; ++i) cout << setw(6) << results[i] << " |";

    cout << "\n\n| Результат |";
    for (int i = 0; i < 10; ++i) cout << setw(6) << statuses[i] << " |";

    cout << "\n\n|     K     |";
    for (int i = 0; i < 10; ++i) cout << setw(6) << attempts[i] << " |";

    return 0;
}