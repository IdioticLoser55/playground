import java.util.*;
import java.util.stream.Stream;
import java.io.*;

public class GiantSquid
{
    public static void main(String[] args)
    {
        String[] input = Advent2021.fileInput("input1.txt");
        System.out.println("Task1: " + task1(input));
        System.out.println("Task2: " + task2(input));

    }

    public static int task1(String[] input)
    {
        Scanner userInput = new Scanner(System.in);

        int drawOrder[] = Stream.of(input[0].split(",")).mapToInt(Integer::parseInt).toArray();
        int boards[][] = new int[(input.length - 1)/ 6][25];

        for(int i = 0; i < boards.length; i++)
        {
            for(int j = 0; j < 5; j++)
            {
                System.arraycopy(
                        Stream.of(input[2 + (i * 6) + j].trim().split("\\s+")).mapToInt(Integer::parseInt).toArray(),
                        0, boards[i], j * 5, 5);
            }
        }
        
//        for(int i = 0; i < boards.length; i++)
//        {
//            for(int j = 0; j < boards[i].length; j++)
//            {
//                System.out.print(" " + boards[i][j]);
//            }
//
//            System.out.println();
//        }

        int countDraw = -1;
        int countBoard = -1;
        int countElements = 0;
        int countRow, countColumn;
        int markCount = 0;
        while (markCount != 5  && ++countDraw < drawOrder.length)
        {
            System.out.println(drawOrder[countDraw]);
            countBoard = -1;
            while(markCount != 5 && ++countBoard < boards.length)
            {
                for(int row = 0; row < 5; row++)
                {
                    for(int col = 0; col < 5; col++)
                    {
                        System.out.print(" " + boards[countBoard][row * 5 + col]);
                    }
                    System.out.println();
                }
                System.out.println();

                for(countElements = 0; countElements < boards[countBoard].length; countElements++)
                {
                    if(boards[countBoard][countElements] == drawOrder[countDraw])
                    {
                        boards[countBoard][countElements] = -1;
                    }
                }

                for(int row = 0; row < 5; row++)
                {
                    for(int col = 0; col < 5; col++)
                    {
                        System.out.print(" " + boards[countBoard][row * 5 + col]);
                    }
                    System.out.println();
                }
                System.out.println("\n");
//                userInput.nextLine();

                markCount = 0;
                countRow = 0; countColumn = 0;
                while(markCount != 5 && countRow < 5)
                {
                    markCount = 0;
                    countColumn = 0;
                    while(countColumn < 5)
                    {
                        if(boards[countBoard][countRow * 5 + countColumn] == -1)
                        {
                            markCount++;
                        }
                        countColumn++;
                    };
                    countRow++;
                };
                countRow = 0; countColumn = 0;
                while(markCount != 5 && countColumn < 5)
                {
                    markCount = 0;
                    countRow = 0;
                    while(countRow < 5)
                    {
                        if(boards[countBoard][countRow * 5 + countColumn] == -1)
                        {
                            markCount++;
                        }
                        countRow++;
                    };
                    countColumn++;
                };
            };
        };

        markCount = 0;
        for(countElements = 0; countElements < boards[countBoard].length; countElements++)
        {
            if(boards[countBoard][countElements] != -1)
            {
                markCount += boards[countBoard][countElements];
            }
        }
        markCount *= drawOrder[countDraw];
        return markCount;
    }
    public static int task2(String[] input)
    {
        Scanner userInput = new Scanner(System.in);

        int drawOrder[] = Stream.of(input[0].split(",")).mapToInt(Integer::parseInt).toArray();
        int boards[][] = new int[(input.length - 1)/ 6][25];

        for(int i = 0; i < boards.length; i++)
        {
            for(int j = 0; j < 5; j++)
            {
                System.arraycopy(
                        Stream.of(input[2 + (i * 6) + j].trim().split("\\s+")).mapToInt(Integer::parseInt).toArray(),
                        0, boards[i], j * 5, 5);
            }
        }
        
        int countDraw = -1;
        int countBoard = -1;
        int countElements = 0;
        int countRow, countColumn;
        int markCount = 0;
        Stack<Integer> winStack = new Stack<Integer>();
        int lastWinScore = 0;
        while (++countDraw < drawOrder.length)
        {
            countBoard = -1;
            while(++countBoard < boards.length)
            {
                for(countElements = 0; countElements < boards[countBoard].length; countElements++)
                {
                    if(boards[countBoard][countElements] == drawOrder[countDraw])
                    {
                        boards[countBoard][countElements] = -1;
                    }
                }

                    if(countBoard == 90)
                    {
                    System.out.println(countBoard);
                    for(int row = 0; row < 5; row++)
                    {
                        for(int col = 0; col < 5; col++)
                        {
                            System.out.print(" " + boards[countBoard][row * 5 + col]);
                        }
                        System.out.println();
                    }
                    System.out.println("\n");
                    }

                markCount = 0;
                countRow = 0; countColumn = 0;
                while(markCount != 5 && countRow < 5)
                {
                    markCount = 0;
                    countColumn = 0;
                    while(countColumn < 5)
                    {
                        if(boards[countBoard][countRow * 5 + countColumn] == -1)
                        {
                            markCount++;
                        }
                        countColumn++;
                    };
                    countRow++;
                };
                countRow = 0; countColumn = 0;
                while(markCount != 5 && countColumn < 5)
                {
                    markCount = 0;
                    countRow = 0;
                    while(countRow < 5)
                    {
                        if(boards[countBoard][countRow * 5 + countColumn] == -1)
                        {
                            markCount++;
                        }
                        countRow++;
                    };
                    countColumn++;
                };
                if(markCount == 5 && !winStack.contains(countBoard))
                {
                    winStack.push(countBoard);

                    markCount = 0;
                    for(countElements = 0; countElements < boards[countBoard].length; countElements++)
                    {
                        if(boards[countBoard][countElements] != -1)
                        {
                            markCount += boards[countBoard][countElements];
                        }
                    }
                    markCount *= drawOrder[countDraw];
                    lastWinScore = markCount;

                }
            };
        };

        return lastWinScore;
    }
}
