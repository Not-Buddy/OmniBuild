import java.util.Arrays;

public class ArrayStats {
    public static double sum(double[] arr) {
        double total = 0;
        for (double num : arr) total += num;
        return total;
    }

    public static double mean(double[] arr) {
        return arr.length > 0 ? sum(arr) / arr.length : 0;
    }

    public static double min(double[] arr) {
        if (arr.length == 0) return 0;
        double minimum = arr[0];
        for (double num : arr) if (num < minimum) minimum = num;
        return minimum;
    }

    public static double max(double[] arr) {
        if (arr.length == 0) return 0;
        double maximum = arr[0];
        for (double num : arr) if (num > maximum) maximum = num;
        return maximum;
    }

    public static double median(double[] arr) {
        if (arr.length == 0) return 0;
        double[] sorted = arr.clone();
        Arrays.sort(sorted);
        int n = sorted.length;
        return (n % 2 == 0) ? (sorted[n/2-1] + sorted[n/2])/2.0 : sorted[n/2];
    }

    public static double stddev(double[] arr) {
        return Math.sqrt(variance(arr));
    }

    public static double variance(double[] arr) {
        if (arr.length <= 1) return 0;
        double avg = mean(arr);
        double sumSquaredDiff = 0;
        for (double num : arr) sumSquaredDiff += Math.pow(num - avg, 2);
        return sumSquaredDiff / (arr.length - 1);
    }
}
