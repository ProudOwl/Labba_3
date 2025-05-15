#include <iostream>
#include <cmath>
using namespace std;

int gsd(int a, int b) {
    while (b != 0) {
        int r = a % b;
        a = b;
        b = r;
    }
    return a;
}

int main() {
    int a, b;
    cin >> a >> b;
    if (b == 1) {
        cout << "Infinity" << endl;
    }

    int numerator, denominator;

    switch (a) {
        case 1:
            numerator = b;
            denominator = pow(b - 1, 2);
            break;
        case 2:
            numerator = b * (b + 1);
            denominator = pow(b - 1, 3);
            break;
        default:
            cout << "Irratoinal" << endl;
            return 0;
    }

    int divider = gsd(numerator, denominator);
    numerator /= divider;
    denominator /= divider;
    cout << numerator << "/" << denominator << endl;
    return 0;
}