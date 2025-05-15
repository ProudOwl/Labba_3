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
        case 3:
            numerator = b * (pow(b, 2) + 4*b + 1);
            denominator = pow(b - 1, 4);
            break;
        case 4:
            numerator = b * (pow(b, 3) + 11*pow(b, 2) + 11*b + 1);
            denominator = pow(b - 1, 5);
            break;
        case 5:
            numerator = b * (pow(b, 4) + 26*pow(b, 3) + 66*pow(b, 2) + 26*b + 1);
            denominator = pow(b - 1, 6);
            break;
        case 6:
            numerator = b * (pow(b, 5) + 57*pow(b, 4) + 302*pow(b, 3) + 302*pow(b, 2) + 57*b + 1);
            denominator = pow(b - 1, 7);
            break;
        case 7:
            numerator = b * (pow(b, 6) + 120*pow(b, 5) + 1191*pow(b, 4) + 2416*pow(b, 3) + 1191*pow(b, 2) + 120*b + 1);
            denominator = pow(b - 1, 8);
            break;
        case 8:
            numerator = b * (pow(b, 7) + 247*pow(b, 6) + 4293*pow(b, 5) + 15619*pow(b, 4) + 15619*pow(b, 3) + 4293*pow(b, 2) + 247*b + 1);
            denominator = pow(b - 1, 9);
            break;
        case 9:
            numerator = b * (pow(b, 8) + 502*pow(b, 7) + 14608*pow(b, 6) + 88234*pow(b, 5) + 156190*pow(b, 4) + 88234*pow(b, 3) + 14608*pow(b, 2) + 502*b + 1);
            denominator = pow(b - 1, 10);
            break;
        case 10:
            numerator = b * (pow(b, 9) + 1013*pow(b, 8) + 47840*pow(b, 7) + 455192*pow(b, 6) + 1310354*pow(b, 5) + 1310354*pow(b, 4) + 455192*pow(b, 3) + 47840*pow(b, 2) + 1013*b + 1);
            denominator = pow(b - 1, 11);
            break;
    }

    int divider = gsd(numerator, denominator);
    numerator /= divider;
    denominator /= divider;
    cout << numerator << "/" << denominator << endl;
    return 0;
}
