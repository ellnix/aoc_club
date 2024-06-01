import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Scanner;

public class Main {
    public static int solveMaze(ArrayList<Integer> maze, boolean isPart2) {
        int steps = 0;
        int currentIndex = 0;

        while (currentIndex >= 0 && currentIndex < maze.size()) {
            int offset = maze.get(currentIndex);
            if (isPart2 && offset >= 3) {
                maze.set(currentIndex, offset - 1);
            } else {
                maze.set(currentIndex, offset + 1);
            }
            currentIndex += offset;
            steps++;
        }

        return steps;
    }

    public static void main(String[] args) {
        ArrayList<Integer> maze = readInputData("src/main/java/input.txt");

        int stepsPart1 = solveMaze(new ArrayList<>(maze), false);
        int stepsPart2 = solveMaze(new ArrayList<>(maze), true);

        System.out.println("Steps to reach the exit (Part 1): " + stepsPart1);
        System.out.println("Steps to reach the exit (Part 2): " + stepsPart2);
    }

    public static ArrayList<Integer> readInputData(String fileName) {
        ArrayList<Integer> inputData = new ArrayList<>();
        try (Scanner scanner = new Scanner(new File(fileName))) {
            while (scanner.hasNextInt()) {
                inputData.add(scanner.nextInt());
            }
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        }
        return inputData;
    }
}
