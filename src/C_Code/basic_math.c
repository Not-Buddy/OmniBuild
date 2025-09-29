// src/C_Code/basic_math.c
#include "basic_math.h"

double c_add(double a, double b) {
    return a + b;
}

double c_subtract(double a, double b) {
    return a - b;
}

double c_multiply(double a, double b) {
    return a * b;
}

double c_divide(double a, double b) {
    if (b == 0.0) {
        return 0.0; // Handle division by zero
    }
    return a / b;
}
