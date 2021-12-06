import java.io.*;
import java.util.*;

//cspell: ignore lanternfish
public class Lanternfish
{
    public static void main(String[] args)
    {
        String[] input = Advent2021.fileInput("input.txt");
        System.out.println("Task1: " + task1(input));
        System.out.println("Task2: " + task2(input));
    }

    public static int task1(String[] input)
    {
        input = input[0].split(",");
        int working = 0, size = 0;
        Queue<Integer> fishes = new LinkedList<Integer>();
        for(int i = 0; i < input.length; i++)
        {
            fishes.add(Integer.parseInt(input[i]));
        }

        for(int i = 0; i < 80; i++)
        {
            size = fishes.size();
            for(int j = 0; j < size; j++)
            {
                working = fishes.remove();
                if(working == 0)
                {
                    fishes.add(8);
                    working = 6;
                }
                else
                {
                    working--;
                }

                fishes.add(working);
            }

//            System.out.println("Day: " + i);
//            size = fishes.size();
//            for(int j = 0; j < size; j++)
//            {
//                working = fishes.remove();
//                System.out.print("," + working);
//                fishes.add(working);
//            }
//            System.out.println("\n");
        }

        return fishes.size();
    }

    public static long task2(String[] input)
    {
        input = input[0].split(",");
        long last = 0, current = 0;
        long[] fishes = new long[9];
        for(int i = 0; i < fishes.length; i++)
        {
            fishes[i] = 0;
        }
        for(int i = 0; i < input.length; i++)
        {
            fishes[Integer.parseInt(input[i])]++;
        }

        for(int j = 0; j < 256; j++)
        {
            fishes[7] += fishes[0];
            current = fishes[0];
            for(int i = fishes.length - 1; i >= 0; i--)
            {
                last = current;
                current = fishes[i];
                fishes[i] = last;
            }
        }

        current = 0;
        for(int i = 0; i < fishes.length; i++)
        {
            current+= fishes[i];
        }

        return current;
    }
}
