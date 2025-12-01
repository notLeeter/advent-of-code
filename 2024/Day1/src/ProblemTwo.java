import java.io.File;
import java.util.*;

public class ProblemTwo {
    public static void main(String[] args) {
        File file = new File("Day1/input.txt");

        HashMap<Integer, Integer> cache = new HashMap<>();
        ArrayList<Integer> left = new ArrayList<>();
        ArrayList<Integer> right = new ArrayList<>();

        try {
            Scanner scanner = new Scanner(file);
            scanner.useDelimiter("\n");
            while (scanner.hasNext()) {
                String line = scanner.next();
                String[] parsed = line.split("\\s+");

                left.add(Integer.parseInt(parsed[0]));
                right.add(Integer.parseInt(parsed[1]));
            }
        } catch (Exception e) {
            e.printStackTrace();
        }

        int score = 0;
        for (int value : left) {
            if (cache.containsKey(value)) {
                score += cache.get(value);
                continue;
            }

            int multi = 0;
            for (Integer integer : right) {
                if (Objects.equals(integer, value)) {
                    ++multi;
                }
            }
            cache.put(value, value * multi);
            score += value * multi;
        }
        System.out.println(score);
    }
}
