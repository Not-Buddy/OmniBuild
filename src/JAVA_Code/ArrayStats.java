// src/JAVA_Code/ArrayStats.java
import java.util.Arrays;

public class ArrayStats {
    public static void main(String[] args) {
        if (args.length == 0) {
            System.out.println("Usage: java ArrayStats <number1> <number2> ...");
            return;
        }
        
        double[] numbers = new double[args.length];
        for (int i = 0; i < args.length; i++) {
            numbers[i] = Double.parseDouble(args[i]);
        }
        
        System.out.println("📊 Array Statistics Analysis");
        System.out.println("═══════════════════════════");
        System.out.println("Input Array: " + Arrays.toString(numbers));
        System.out.println("Count: " + numbers.length);
        System.out.println();
        
        System.out.println("📈 Basic Statistics:");
        System.out.printf("Sum: %.2f%n", sum(numbers));
        System.out.printf("Average: %.2f%n", average(numbers));
        System.out.printf("Min: %.2f%n", min(numbers));
        System.out.printf("Max: %.2f%n", max(numbers));
        System.out.printf("Range: %.2f%n", max(numbers) - min(numbers));
        System.out.println();
        
        System.out.println("📏 Advanced Statistics:");
        System.out.printf("Median: %.2f%n", median(numbers));
        System.out.printf("Standard Deviation: %.2f%n", standardDeviation(numbers));
        System.out.printf("Variance: %.2f%n", variance(numbers));
    }
    
    public static double sum(double[] arr) {
        double total = 0;
        for (double num : arr) total += num;
        return total;
    }
    
    public static double average(double[] arr) {
        return arr.length > 0 ? sum(arr) / arr.length : 0;
    }
    
    public static double min(double[] arr) {
        if (arr.length == 0) return 0;
        double minimum = arr[0];
        for (double num : arr) {
            if (num < minimum) minimum = num;
        }
        return minimum;
    }
    
    public static double max(double[] arr) {
        if (arr.length == 0) return 0;
        double maximum = arr[0];
        for (double num : arr) {
            if (num > maximum) maximum = num;
        }
        return maximum;
    }
    
    public static double median(double[] arr) {
        if (arr.length == 0) return 0;
        double[] sorted = arr.clone();
        Arrays.sort(sorted);
        int n = sorted.length;
        
        if (n % 2 == 0) {
            return (sorted[n/2 - 1] + sorted[n/2]) / 2.0;
        } else {
            return sorted[n/2];
        }
    }
    
    public static double standardDeviation(double[] arr) {
        return Math.sqrt(variance(arr));
    }
    
    public static double variance(double[] arr) {
        if (arr.length <= 1) return 0;
        double avg = average(arr);
        double sumSquaredDiff = 0;
        
        for (double num : arr) {
            sumSquaredDiff += Math.pow(num - avg, 2);
        }
        
        return sumSquaredDiff / (arr.length - 1);
    }
}
