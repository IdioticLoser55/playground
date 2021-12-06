import java.util.*;
import java.util.stream.Stream;
import java.io.*;

public class HydrothermalVenture
{
    public static void main(String[] args)
    {
        //I think this will work.
        //String.split(",|( -> )")
        String[] input = Advent2021.fileInput("input.txt");
        System.out.println("Task 1: " + task1(input));
        System.out.println("Task 2: " + task2(input));

    }

    public static int task1(String[] input)
    {
        int x, y, i;
        int count = 0;
        int[][] points = new int[input.length][];
        int[][] grid = new int[1000][1000];
        for(i = 0; i < input.length; i++)
        {
            points[i] = Stream.of(input[i].split(",| -> ")).mapToInt(Integer::parseInt).toArray();
        }
        for(y = 0; y < 1000; y++)
        {
            for(x = 0; x < 1000; x++)
            {
                grid[x][y] = 0;
            }
        }

        for(i = 0; i < points.length; i++)
        {
            if(points[i][0] == points[i][2])
                //vertical line.
            {
                for(y = points[i][1] < points[i][3] ? points[i][1] : points[i][3];
                        y <= (points[i][1] > points[i][3] ? points[i][1] : points[i][3]); y++)
                {
                    grid[points[i][0]][y]++;
                }
            }
            else if(points[i][1] == points[i][3])
                //horizontal line.
            {
                for(x = points[i][0] < points[i][2] ? points[i][0] : points[i][2];
                        x <= (points[i][0] > points[i][2] ? points[i][0] : points[i][2]); x++)
                {
                    grid[x][points[i][1]]++;
                }

            }
        }
        for(y = 0; y < 1000; y++)
        {
            for(x = 0; x < 1000; x++)
            {
                if(grid[x][y] >= 2)
                {
                    count++;
                };
            }
        }
        return count++;
    }
    public static int task2(String[] input)
    {
        int x, y, i;
        int count = 0;
        int[][] points = new int[input.length][];
        int[][] grid = new int[1000][1000];
        for(i = 0; i < input.length; i++)
        {
            points[i] = Stream.of(input[i].split(",| -> ")).mapToInt(Integer::parseInt).toArray();
        }
        for(y = 0; y < 1000; y++)
        {
            for(x = 0; x < 1000; x++)
            {
                grid[x][y] = 0;
            }
        }


        for(i = 0; i < points.length; i++)
        {
            if(points[i][0] == points[i][2])
                //vertical line.
            {
                for(y = points[i][1] < points[i][3] ? points[i][1] : points[i][3];
                        y <= (points[i][1] > points[i][3] ? points[i][1] : points[i][3]); y++)
                {
                    grid[points[i][0]][y]++;
                }
            }
            else if(points[i][1] == points[i][3])
                //horizontal line.
            {
                for(x = points[i][0] < points[i][2] ? points[i][0] : points[i][2];
                        x <= (points[i][0] > points[i][2] ? points[i][0] : points[i][2]); x++)
                {
                    grid[x][points[i][1]]++;
                }

            }
            else
                //diagonal lines.
            {
                x = points[i][0]; y = points[i][1];
                do
                {
                    grid[x][y]++;
                    if(points[i][0] > points[i][2])
                    {
                        x--;
                    }
                    else
                    {
                        x++;
                    }
                    if(points[i][1] > points[i][3])
                    {
                        y--;
                    }
                    else
                    {
                        y++;
                    }
                }while(x != points[i][2]);
                grid[x][y]++;
            }
        }


        for(y = 0; y < 1000; y++)
        {
            for(x = 0; x < 1000; x++)
            {
                if(grid[x][y] >= 2)
                {
                    count++;
                };
            }
        }
        return count++;
    }
}
