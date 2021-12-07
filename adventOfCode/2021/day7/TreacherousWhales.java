import java.util.*;
import java.util.stream.Stream;
import java.io.*;

public class TreacherousWhales
{
    public static void main(String[] args)
    {
        String[] input = Advent2021.fileInput("input.txt");
        System.out.println("Task 1: " + task1(input));
        System.out.println("Task 2: " + task2(input));
    }
    public static int task1(String[] input)
    {
        int temp = 0;
        int mid1, mid2;
        int sum1 = 0, sum2 = 0;
        int[] positions = Stream.of(input[0].split(",")).mapToInt(Integer::parseInt).toArray();
        
        for(int i = 1; i < positions.length; i++)
        {
            for(int j = 0; j < positions.length - i; j++)
            {
                if(positions[j] > positions[j + 1])
                {
                    temp = positions[j];
                    positions[j] = positions[j + 1];
                    positions[j + 1] = temp;
                }
            }
        }

        temp = positions.length / 2;
        mid1 = positions[temp];
        mid2 = positions[temp + 1];

        for(int i = 0; i < positions.length; i++)
        {
            sum1 += Math.abs(mid1 - positions[i]);
            sum2 += Math.abs(mid2 - positions[i]);
        }

        return sum1 < sum2 ? sum1 : sum2;
    }

    public static long task2(String[] input)
    {
        long avg1 = 0, avg2 = 0;
        long temp, sum1 = 0, sum2 = 0;
        int[] positions = Stream.of(input[0].split(",")).mapToInt(Integer::parseInt).toArray();
        
        for(int i = 0; i < positions.length; i++)
        {
            avg1 += positions[i];
        }

        avg1 = (int)Math.floor((double)avg1 / positions.length);
        avg2 = avg1 + 1;

        System.out.println("Avg1: " + avg1);
        System.out.println("Avg2: " + avg2);

        for(int i = 0; i < positions.length; i++)
        {
            temp = Math.abs(avg1 - positions[i]);
            sum1 += temp * (temp + 1) / 2;
            temp = Math.abs(avg2 - positions[i]);
            sum2 += temp * (temp + 2) / 2;
        }

        return sum1 < sum2 ? sum1 : sum2;
    }
}
