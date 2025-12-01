import java.io.File;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Scanner;

public class ProblemOne {
    public static void main(String[] args) {
        File file = new File("Day1/input.txt");

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

        Collections.sort(left);
        Collections.sort(right);

        int totalDifference = 0;
        for (int i = 0; i < left.size(); i++) {
            totalDifference += Math.abs(left.get(i) - right.get(i));
        }
        System.out.println(totalDifference);
    }
}