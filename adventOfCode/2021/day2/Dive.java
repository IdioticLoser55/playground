import java.io.*;
import java.util.*;

public class Dive
{
    public static void main(String[] args)
    { 
        String[] input = fileInput("input1.txt");
        System.out.println("Task1: " + task1(input));
        System.out.println("Task2: " + task2(input));
    }

    public static int task1(String[] input)
    {
        int depth = 0;
        int distance = 0;

        String[] split;

        for(String line: input)
        {
            split = line.split(" ");

            switch(split[0])
            {
                case "forward":
                    distance += Integer.parseInt(split[1]);
                    break;
                case "down":
                    depth += Integer.parseInt(split[1]);
                    break;
                case "up":
                    depth -= Integer.parseInt(split[1]);
                    break;
            }
        }

        return depth * distance;
    }

    public static int task2(String[] input)
    {
        int aim = 0;
        int depth = 0;
        int hDistance = 0;

        String[] split;

        for(String line: input)
        {
            split = line.split(" ");

            switch(split[0])
            {
                case "forward":
                    hDistance += Integer.parseInt(split[1]);
                    depth += aim * Integer.parseInt(split[1]);
                    break;
                case "down":
                    aim += Integer.parseInt(split[1]);
                    break;
                case "up":
                    aim -= Integer.parseInt(split[1]);
                    break;
            }
        }

        return depth * hDistance;
    }

    public static String[] fileInput(String FileName)
    {
        List<String>input = new LinkedList<String>();
        try
        {
            File myObj = new File("input1.txt");
            Scanner myReader = new Scanner(myObj);

            while(myReader.hasNextLine())
            {
                input.add(myReader.nextLine());
            }

            myReader.close();
        }
        catch(FileNotFoundException e)
        {
            System.out.println("An Error Occurred: ");
            e.printStackTrace();
        }

        return input.toArray(new String[input.size()]);
    }
}
