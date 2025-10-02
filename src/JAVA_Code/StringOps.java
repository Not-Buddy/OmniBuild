
public class StringOps {

    public static int length(String text) {
        return text.length();
    }

    public static String uppercase(String text) {
        return text.toUpperCase();
    }

    public static String lowercase(String text) {
        return text.toLowerCase();
    }

    public static String reverse(String str) {
        return new StringBuilder(str).reverse().toString();
    }

    public static boolean isPalindrome(String str) {
        String clean = str.replaceAll("[^a-zA-Z0-9]", "").toLowerCase();
        return clean.equals(reverse(clean));
    }
    
    public static int wordCount(String str) {
        if (str == null || str.trim().isEmpty()) return 0;
        return str.trim().split("\\s+").length;
    }
}
