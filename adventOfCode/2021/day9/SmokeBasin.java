import java.util.*;
import java.util.stream.Stream;
import java.io.*;

public class SmokeBasin
{
    public static void main(String[] args)
    {
        String[] input = Advent2021.fileInput("test.txt");
        System.out.println("Test 1: " + task1(input));
        System.out.println("Test 2: " + task2(input));
        input = Advent2021.fileInput("input.txt");
        System.out.println("Test 1: " + task1(input));
        System.out.println("Test 2: " + task2(input));
    }
    public static int task1(String[] input)
    {
        int sum = 0;
        char[] row;
        int[][] map = new int[input[0].length() + 2][input.length + 2];
        int[][] mapClone;

        for(int y = 0; y < input.length; y++)
        {
            row = input[y].toCharArray();
            for(int x = 0; x < row.length; x++)
            {
                map[x + 1][y + 1] = Integer.parseInt(String.valueOf(row[x]));
            }
        }
        for(int x = 0; x < map.length; x++)
        {
            map[x][0] = 10;
            map[x][map[0].length - 1] = 10;
        }
        for(int y = 0; y < map[0].length; y++)
        {
            map[0][y] = 10;
            map[map.length - 1][y] = 10;
        }
        mapClone = new int[map.length][map[0].length];
        for(int y = 1; y < map[0].length - 1; y++)
        {
            for(int x = 1; x < map.length - 1; x++)
            {
                mapClone[x][y] = map[x][y];
            }
        }

        for(int y = 1; y < map[0].length - 1; y++)
        {
            for(int x = 1; x < map.length - 1; x++)
            {
                if(map[x][y] >= map[x + 1][y])
                {
                    mapClone[x][y] = 10;
                }
                if(map[x][y] >= map[x - 1][y])
                {
                    mapClone[x][y] = 10;
                }
                if(map[x][y] >= map[x][y + 1])
                {
                    mapClone[x][y] = 10;
                }
                if(map[x][y] >= map[x][y - 1])
                {
                    mapClone[x][y] = 10;
                }
            }
        }


        for(int y = 1; y < mapClone[0].length - 1; y++)
        {
            for(int x = 1; x < mapClone.length - 1; x++)
            {
                if(mapClone[x][y] != 10)
                {
                    sum += mapClone[x][y] + 1;
                }
            }
        }

        return sum;
    }

    public static int task2(String[] input)
    {
        int sum = 0;
        char[] row;
        int[][] map = new int[input[0].length() + 2][input.length + 2];
        int[][] mapLow;
        int[][] mapClone;
        int[] topThree = new int[3];

        for(int y = 0; y < input.length; y++)
        {
            row = input[y].toCharArray();
            for(int x = 0; x < row.length; x++)
            {
                map[x + 1][y + 1] = Integer.parseInt(String.valueOf(row[x]));
            }
        }
        for(int x = 0; x < map.length; x++)
        {
            map[x][0] = 10;
            map[x][map[0].length - 1] = 10;
        }
        for(int y = 0; y < map[0].length; y++)
        {
            map[0][y] = 10;
            map[map.length - 1][y] = 10;
        }
        mapLow = new int[map.length][map[0].length];
        for(int y = 0; y < map[0].length; y++)
        {
            for(int x = 0; x < map.length; x++)
            {
                mapLow[x][y] = map[x][y];
            }
        }

        for(int y = 1; y < map[0].length - 1; y++)
        {
            for(int x = 1; x < map.length - 1; x++)
            {
                if(map[x][y] >= map[x + 1][y])
                {
                    mapLow[x][y] = 10;
                }
                if(map[x][y] >= map[x - 1][y])
                {
                    mapLow[x][y] = 10;
                }
                if(map[x][y] >= map[x][y + 1])
                {
                    mapLow[x][y] = 10;
                }
                if(map[x][y] >= map[x][y - 1])
                {
                    mapLow[x][y] = 10;
                }
            }
        }


        for(int y = 1; y < mapLow[0].length - 1; y++)
        {
            for(int x = 1; x < mapLow.length - 1; x++)
            {
                if(mapLow[x][y] != 10)
                {
                    mapClone = emptyMap(map.length, map[0].length, 10);
                    mapClone = markBasin(map, mapClone, x, y);

                    sum = 0;
                    for(int y1 = 0; y1 < map[0].length; y1++)
                    {
                        for(int x1 = 0; x1 < map.length; x1++)
                        {
                            if(mapClone[x1][y1] < 10)
                            {
                                sum += 1;
                            }
                        }
                    }

                    System.out.println(sum);
                    if(sum > topThree[2])
                    {
                        if(sum > topThree[1])
                        {
                            if(sum > topThree[0])
                            {
                                topThree[2] = topThree[1];
                                topThree[1] = topThree[0];
                                topThree[0] = sum;
                            }
                            else
                            {
                                topThree[2] = topThree[1];
                                topThree[1] = sum;
                            }
                        }
                        else
                        {
                            topThree[2] = sum;
                        }
                    }
                }
            }
        }

        return topThree[0] * topThree[1] * topThree[2];
    }

    private static int[][] markBasin(int[][] map, int[][] mapClone, int x, int y)
    {
        mapClone[x][y] = map[x][y];

        if(map[x][y] < map[x + 1][y] && map[x + 1][y] < 9)
        {
            mapClone = markBasin(map, mapClone, x + 1, y);
        }
        if(map[x][y] < map[x - 1][y] && map[x - 1][y] < 9)
        {
            mapClone = markBasin(map, mapClone, x - 1, y);
        }
        if(map[x][y] < map[x][y + 1] && map[x][y + 1] < 9)
        {
            mapClone = markBasin(map, mapClone, x, y + 1);
        }
        if(map[x][y] < map[x][y - 1] && map[x][y - 1] < 9)
        {
            mapClone = markBasin(map, mapClone, x, y - 1);
        }

        return mapClone;
    }

    private static int[][] emptyMap(int X, int Y, int fill)
    {
        int[][] newMap = new int[X][Y];
        for(int y = 0; y < Y; y++)
        {
            for(int x = 0; x < X; x++)
            {
                newMap[x][y] = fill;
            }
        }

        return newMap;
    }
}
