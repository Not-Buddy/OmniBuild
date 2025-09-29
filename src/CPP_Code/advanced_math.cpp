// src/CPP_Code/advanced_math.cpp
#include "advanced_math.h"
#include <cmath>

extern "C" {
    double cpp_power(double base, double exponent) {
        return std::pow(base, exponent);
    }

    double cpp_sqrt(double x) {
        if (x < 0.0) return -1.0; // Error indicator
        return std::sqrt(x);
    }

    double cpp_sin(double x) {
        return std::sin(x);
    }

    double cpp_cos(double x) {
        return std::cos(x);
    }

    double cpp_tan(double x) {
        return std::tan(x);
    }

    double cpp_log(double x) {
        if (x <= 0.0) return -1.0; // Error indicator
        return std::log(x);
    }
}
