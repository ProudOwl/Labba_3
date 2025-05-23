#include <iostream>
#include <vector>
#include <iomanip>
#include <random>
#include <cmath>
#include <algorithm>
#include <thread>
#include <mutex>
#include <string>
#include <sstream>
using namespace std;

int RandomGenerator(int min, int max) {
    random_device rd;
    mt19937 gen(rd());
    uniform_int_distribution<int> dis(min, max);
    return dis(gen);
}

bool IsPrime(int i, vector<int>& prime) {
    for (int j = 0; j < prime.size(); j++) {
        if (i % prime[j] == 0) {
            return false;
        }
    }
    return true;
}

vector<int> IsPrimeEratosfen(int p) {
    vector<int> primeNumb;
    primeNumb.push_back(2);
    for (int i = 3; i < p; i += 2) {
        if (IsPrime(i, primeNumb)) {
            primeNumb.push_back(i);
        }
    }
    return primeNumb;
}

uint64_t mod(uint64_t number, uint64_t power, uint64_t n) {
    uint64_t ostatok = 1;
    while (power > 0) {
        if (power % 2 != 0) {
            ostatok = (ostatok * number) % n;
        }
        power /= 2;
        number = (number * number) % n;
    }
    return ostatok;
}

bool TeoremaFerma(int a, int x, int p) {
    uint64_t result = mod(a, x, p);
    return result == 1;
}

bool MillerTest(int64_t n, const vector<int>& factors, int t) {
    for (int i = 0; i < t; i++) {
        int a = RandomGenerator(2, n - 1);
        if (!TeoremaFerma(a, n - 1, n)) {
            return false;
        }
    }

    for (int q : factors) {
        bool all_a_equal_one = true;

        for (int i = 0; i < t; i++) {
            int a = RandomGenerator(2, n - 1);
            if (!TeoremaFerma(a, (n - 1) / q, n)) {
                all_a_equal_one = false;
                break;
            }
        }

        if (all_a_equal_one) {
            return false;
        }
    }

    return true;
}

int countBits(uint64_t number) {
    if (number == 0) return 1;
    return static_cast<int>(log2(number) + 1);
}

int isWithinBitRange(uint64_t number, int bitCount, int formin) {
    int numBits = countBits(number);
    if (numBits < bitCount) return 1;
    if (numBits > bitCount - formin) return 2;
    return 3;
}

bool createNum(int k, vector<int>& numbers, vector<string>& results, vector<int>& attempts, vector<int>& primes) {
    vector<int> q;
    int64_t m = 1;
    int tempM = 1;
    int qi;
    int ai;
    int64_t n;
    int targetBitCountM = k - 1;
    int tempOshibki = 0;
    int formin = 0;
    
    if (k < 7 && k > 4) {
        formin = -1;
    }
    else if (k == 4) {
        formin = -2;
    }

    while (true) {
        qi = primes[RandomGenerator(0, primes.size() / 10)];
        ai = RandomGenerator(1, 12);
        tempM *= pow(qi, ai);
        if (isWithinBitRange(tempM, targetBitCountM, formin) == 2) {
            m = 1;
            tempM = 1;
            q.clear();
        }
        else if (isWithinBitRange(tempM, targetBitCountM, formin) == 1) {
            q.push_back(qi);
        }
        else {
            m = tempM;
            q.push_back(qi);
            n = 2 * m + 1;
            if (MillerTest(n, q, 1)) {
                break;
            }
            else {
                tempOshibki++;
                m = 1;
                tempM = 1;
                n = 0;
                q.clear();
            }
        }
    }

    // Проверка на уникальность числа
    while (!numbers.empty() && find(numbers.begin(), numbers.end(), n) != numbers.end()) {
        m = 1;
        tempM = 1;
        q.clear();
        while (true) {
            qi = primes[RandomGenerator(0, primes.size() / 10)];
            ai = RandomGenerator(1, 12);
            tempM *= pow(qi, ai);
            if (isWithinBitRange(tempM, targetBitCountM, formin) == 2) {
                m = 1;
                tempM = 1;
                q.clear();
            }
            else if (isWithinBitRange(tempM, targetBitCountM, formin) == 1) {
                q.push_back(qi);
            }
            else {
                m = tempM;
                q.push_back(qi);
                n = 2 * m + 1;
                if (MillerTest(n, q, 1)) {
                    break;
                }
                else {
                    tempOshibki++;
                    m = 1;
                    tempM = 1;
                    n = 0;
                    q.clear();
                }
            }
        }
    }

    int tempOshibki2 = 0;
    int i;
    int iter = 100;
    while (true) {
        for (i = 0; i < iter; i++) {
            if (!MillerTest(n, q, 1)) {
                tempOshibki2++;
            }
        }
        if (tempOshibki2 <= i / 10) {
            break;
        }
        else {
            tempOshibki2 = 0;
            iter -= 10;
            if (iter < 10) {
                iter = 10;
            }
        }
    }

    if (tempOshibki2 <= i / 10) {
        results.push_back("+");
    }
    else {
        results.push_back("-");
    }
    numbers.push_back(n);
    attempts.push_back(tempOshibki);
    return true;
}

int getSingleWordInput(const string& prompt) {
    string input;
    double value;

    while (true) {
        cout << prompt;
        getline(cin, input);

        istringstream iss(input);
        string word;
        if (!(iss >> word) || (iss >> word)) {
            cout << "Неправильное количество. Пожалуйста, введите одно число.\n";
            continue;
        }

        bool valid = true;
        for (size_t i = 0; i < input.size(); ++i) {
            char c = input[i];
            if (!isdigit(c)) {
                valid = false;
                break;
            }
        }
        if (!valid) {
            cout << "Некорректное число. Попробуйте снова.\n";
            continue;
        }
        istringstream(input) >> value;
        if (value <= 3 || value > 20) {
            cout << "Некорректное число. Попробуйте снова.\n";
            continue;
        }
        return value;
    }
}

int main() {
    vector<int> numbers;
    vector<string> Res;
    vector<int> K;
    int t = 0;
    int input;

    while (true) {
        input = getSingleWordInput("Введите размерность простого числа \n");
        break;
    }

    t = input;

    vector<int> primes = IsPrimeEratosfen(500);
    cout << "Простые числа до 500 (Решето Эратосфена):\n" << endl;
    for (size_t i = 0; i < primes.size(); ++i) {
        cout << primes[i] << (i % 10 == 9 ? "\n" : " ");
    }
    cout << "\n\n";

    for (int i = 0; i < 10; i++) {
        createNum(t, numbers, Res, K, primes);
    }

    // Вывод результатов
    if (!numbers.empty()) {
        cout << "\nРезультаты генерации простых чисел (" << t << " бит):\n\n";
        cout << "|    Num    |";
        for (size_t i = 0; i < numbers.size(); ++i) {
            cout << setw(6) << i+1 << " |";
        }
        cout << "\n|-----------|";
        for (size_t i = 0; i < numbers.size(); ++i) {
            cout << "-------|";
        }
        cout << "\n|     P     |";
        for (int num : numbers) {
            cout << setw(6) << num << " |";
        }
        cout << "\n| Результат |";
        for (const string& res : Res) {
            cout << setw(6) << res << " |";
        }
        cout << "\n|     K     |";
        for (int att : K) {
            cout << setw(6) << att << " |";
        }
        cout << endl;
    }

    return 0;
}