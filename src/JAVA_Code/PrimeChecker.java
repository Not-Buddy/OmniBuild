
public class PrimeChecker {
    // Return true if n is prime
    public static boolean isPrime(long n) {
        if (n < 2) return false;
        if (n == 2) return true;
        if (n % 2 == 0) return false;
        for (long i = 3; i * i <= n; i += 2) {
            if (n % i == 0) return false;
        }
        return true;
    }

    // Return the smallest non-trivial factor, or -1 if n is prime
    public static long factor(long n) {
        if (n < 2) return -1;
        if (n % 2 == 0) return 2;
        for (long i = 3; i * i <= n; i += 2) {
            if (n % i == 0) return i;
        }
        return -1;
    }
}
