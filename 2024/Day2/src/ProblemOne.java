import java.io.File;
import java.util.Objects;
import java.util.Scanner;

public class ProblemOne {
    public static void main(String[] args) {
        File file = new File("Day2/input.txt");
        int safeCount = 0;
        try {
            Scanner scanner = new Scanner(file);
            while (scanner.hasNextLine()) {
                String line = scanner.nextLine();
                String[] level = line.split(" ");

                int initialDirection = 0;
                boolean isSafe = false;
                for (int i = 0; i < level.length - 1; i++) {
                    isSafe = false;
                    int left = Integer.parseInt(level[i]);
                    int right = Integer.parseInt(level[i + 1]);

                    if (Objects.equals(left, right)) break;
                    if (Math.abs(left - right) > 3) break;

                    int direction = (left - right) / Math.abs(left - right);
                    if (Objects.equals(initialDirection, 0)) initialDirection = direction;
                    if (!Objects.equals(initialDirection, direction)) break;
                    isSafe = true;
                }
                if (isSafe) safeCount++;
            }
        } catch (Exception e) {
            e.printStackTrace();
        }

        System.out.println(safeCount);
    }
}