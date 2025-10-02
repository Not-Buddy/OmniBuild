// src/JAVA_Code/StringOps.java
public class StringOps {
    public static void main(String[] args) {
        if (args.length == 0) {
            System.out.println("Usage: java StringOps <text>");
            return;
        }
        
        String text = String.join(" ", args);
        
        System.out.println("🔤 String Operations Analysis");
        System.out.println("═════════════════════════════");
        System.out.println("Original Text: \"" + text + "\"");
        System.out.println();
        
        System.out.println("📏 Basic Properties:");
        System.out.println("Length: " + text.length());
        System.out.println("Word Count: " + countWords(text));
        System.out.println("Character Count (no spaces): " + text.replaceAll("\\s", "").length());
        System.out.println();
        
        System.out.println("🔄 Transformations:");
        System.out.println("Uppercase: \"" + text.toUpperCase() + "\"");
        System.out.println("Lowercase: \"" + text.toLowerCase() + "\"");
        System.out.println("Reversed: \"" + reverse(text) + "\"");
        System.out.println("Title Case: \"" + toTitleCase(text) + "\"");
        System.out.println();
        
        System.out.println("🔍 Analysis:");
        System.out.println("Is Palindrome: " + isPalindrome(text));
        System.out.println("Starts with Vowel: " + startsWithVowel(text));
        System.out.println("Contains Numbers: " + containsNumbers(text));
        System.out.println("All Uppercase: " + text.equals(text.toUpperCase()));
        System.out.println("All Lowercase: " + text.equals(text.toLowerCase()));
    }
    
    public static String reverse(String str) {
        return new StringBuilder(str).reverse().toString();
    }
    
    public static boolean isPalindrome(String str) {
        String clean = str.replaceAll("[^a-zA-Z0-9]", "").toLowerCase();
        return clean.equals(reverse(clean));
    }
    
    public static boolean startsWithVowel(String str) {
        if (str.isEmpty()) return false;
        char first = Character.toLowerCase(str.charAt(0));
        return "aeiou".indexOf(first) != -1;
    }
    
    public static boolean containsNumbers(String str) {
        return str.matches(".*\\d.*");
    }
    
    public static int countWords(String str) {
        if (str == null || str.trim().isEmpty()) return 0;
        return str.trim().split("\\s+").length;
    }
    
    public static String toTitleCase(String str) {
        if (str == null || str.isEmpty()) return str;
        
        StringBuilder result = new StringBuilder();
        boolean capitalizeNext = true;
        
        for (char c : str.toCharArray()) {
            if (Character.isWhitespace(c)) {
                capitalizeNext = true;
                result.append(c);
            } else if (capitalizeNext) {
                result.append(Character.toUpperCase(c));
                capitalizeNext = false;
            } else {
                result.append(Character.toLowerCase(c));
            }
        }
        
        return result.toString();
    }
}
