import java.io.File;
import java.util.ArrayList;
import java.util.Scanner;

public class ProblemTwo {
    static int initialDirection = 0;

    public static void main(String[] args) {
        File file = new File("Day2/input.txt");
        int safeCount = 0;

        try (Scanner scanner = new Scanner(file)) {
            while (scanner.hasNextLine()) {
                String line = scanner.nextLine();
                String[] numbers = line.split(" ");
                ArrayList<Integer> level = new ArrayList<>();

                // Parse numbers into the level
                for (String num : numbers) {
                    level.add(Integer.parseInt(num));
                }

                // Check if the level is safe as is
                if (isSafeLevel(level)) {
                    safeCount++;
                    continue;
                }

                // Try removing each level element one at a time
                boolean dampenedSafe = false;
                for (int i = 0; i < level.size(); i++) {
                    ArrayList<Integer> modifiedLevel = new ArrayList<>(level);
                    modifiedLevel.remove(i); // Remove the current element
                    if (isSafeLevel(modifiedLevel)) {
                        dampenedSafe = true;
                        break; // Stop checking if any modified version is safe
                    }
                }

                // If any dampened version is safe, count it as safe
                if (dampenedSafe) {
                    safeCount++;
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }

        System.out.println("Safe levels count: " + safeCount);
    }

    // Check if a level is safe
    public static boolean isSafeLevel(ArrayList<Integer> level) {
        initialDirection = 0; // Reset initial direction
        for (int i = 0; i < level.size() - 1; i++) {
            if (!isSafePair(level.get(i), level.get(i + 1))) {
                return false; // If any pair is unsafe, the level is unsafe
            }
        }
        return true; // All pairs are safe
    }

    // Check if a pair of numbers is safe
    public static boolean isSafePair(int left, int right) {
        if (left == right) return false;
        if (Math.abs(left - right) > 3) return false;

        int direction = (left > right) ? -1 : 1; // Determine direction
        if (initialDirection == 0) initialDirection = direction; // Set initial direction
        return initialDirection == direction;
    }
}
