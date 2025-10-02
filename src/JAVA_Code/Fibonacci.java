// src/JAVA_Code/Fibonacci.java
public class Fibonacci {
    public static void main(String[] args) {
        if (args.length == 0) {
            System.out.println("Usage: java Fibonacci <number>");
            return;
        }
        
        int n = Integer.parseInt(args[0]);
        System.out.println("Fibonacci sequence (first " + n + " numbers):");
        
        for (int i = 0; i < n; i++) {
            System.out.print(fibonacci(i) + " ");
        }
        System.out.println();
        
        if (n > 0) {
            System.out.println("Fibonacci(" + (n-1) + ") = " + fibonacci(n-1));
        }
    }
    
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
    
    // Alternative recursive method (slower but educational)
    public static long fibonacciRecursive(int n) {
        if (n <= 1) return n;
        return fibonacciRecursive(n - 1) + fibonacciRecursive(n - 2);
    }
}
