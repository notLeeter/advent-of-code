import java.io.File;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class ProblemOne {
    public static void main(String[] args) {
        File file = new File("Day3/input.txt");
        int total = 0;
        try {
            Scanner scanner = new Scanner(file);
            while (scanner.hasNextLine()) {
                String line = scanner.nextLine();
                Matcher matcher = Pattern.compile("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").matcher(line);
                while (matcher.find()) {
                    total += Integer.parseInt(matcher.group(1)) * Integer.parseInt(matcher.group(2));
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }

        System.out.println(total);
    }
}