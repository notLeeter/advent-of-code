import java.io.File;
import java.util.Objects;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class ProblemTwo {
    public static void main(String[] args) {
        File file = new File("Day3/input.txt");
        int total = 0;
        try {
            Scanner scanner = new Scanner(file);
            boolean ignore = false;
            while (scanner.hasNextLine()) {
                String line = scanner.nextLine();
                Matcher matcher = Pattern.compile("(do\\(\\)|don't\\(\\)|mul\\(([0-9]{1,3}),([0-9]{1,3})\\))").matcher(line);
                while (matcher.find()) {
                    String match = matcher.group(1);

                    if (Objects.equals(match, "don't()")) ignore = true;
                    else if (Objects.equals(match, "do()")) ignore = false;
                    else if (!ignore) {
                        total += Integer.parseInt(matcher.group(2)) * Integer.parseInt(matcher.group(3));
                    }
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }

        System.out.println(total);
    }
}