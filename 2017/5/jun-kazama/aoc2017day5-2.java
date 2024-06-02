import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Scanner;

public class Main {
    // In the main method, puzzle input is stored in the ArrayList maze.
    public static int solveMaze(ArrayList<Integer> maze) {
        int steps = 0;
        int currentIndex = 0;

        while (currentIndex >= 0 && currentIndex < maze.size()) {
            int offset = maze.get(currentIndex);
            if (offset >= 3) {
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

        int steps = solveMaze(maze);

        System.out.println("Steps to reach the exit: " + steps);
    }

    public static ArrayList<Integer> readInputData(String fileName) {
        ArrayList<Integer> inputData = new ArrayList<>();
        try {
            File file = new File(fileName);
            Scanner scanner = new Scanner(file);
            while (scanner.hasNextInt()) {
                inputData.add(scanner.nextInt());
            }
            scanner.close();
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        }
        return inputData;
    }
}

