
public class Fibonacci {
    // Return n-th Fibonacci number
    public static long fibonacci(int n) {
        if (n <= 1) return n;
        long a = 0, b = 1;
        for (int i = 2; i <= n; i++) {
            long temp = a + b;
            a = b;
            b = temp;
        }
        return b;
    }

    // Return an array with the first n Fibonacci numbers
    public static long[] fibonacciSeq(int n) {
        long[] seq = new long[n];
        for (int i = 0; i < n; i++) {
            seq[i] = fibonacci(i);
        }
        return seq;
    }
}
