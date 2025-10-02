// src/JAVA_Code/PrimeChecker.java
public class PrimeChecker {
    public static void main(String[] args) {
        if (args.length == 0) {
            System.out.println("Usage: java PrimeChecker <number>");
            return;
        }
        
        long num = Long.parseLong(args[0]);
        boolean prime = isPrime(num);
        
        System.out.println("Number: " + num);
        System.out.println("Is Prime: " + prime);
        
        if (prime) {
            System.out.println(num + " is a prime number!");
        } else {
            System.out.println(num + " is not a prime number.");
            if (num > 1) {
                System.out.println("Factors: " + findFactors(num));
            }
        }
    }
    
    public static boolean isPrime(long n) {
        if (n < 2) return false;
        if (n == 2) return true;
        if (n % 2 == 0) return false;
        
        for (long i = 3; i * i <= n; i += 2) {
            if (n % i == 0) return false;
        }
        return true;
    }
    
    public static String findFactors(long n) {
        StringBuilder factors = new StringBuilder();
        for (long i = 2; i <= Math.sqrt(n); i++) {
            if (n % i == 0) {
                factors.append(i).append(", ").append(n/i).append(", ");
                break;
            }
        }
        if (factors.length() > 0) {
            return factors.substring(0, factors.length() - 2);
        }
        return "1, " + n;
    }
}
