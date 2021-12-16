import java.util.*;
import java.util.stream.Stream;
import java.io.*;

public class SyntaxScoring
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
    public static int task1(String[] inputs)
    {
        int countNext = 0, countPrev = 0, countInput = 0;
        int sum = 0;
        boolean corrupt = false;
        Stack<Character> chunks = new Stack<Character>();
        Character[] open = {'(', '[', '{', '<'};
        Character[] close = {')', ']', '}', '>'};


        for(String input: inputs)
        {
            countInput = -1;
            corrupt = false;
            while(!corrupt && ++countInput < input.length())
            {
                if(chunks.size() > 0)
                    // active chunk.
                {
                    countNext = 0;
                    while(open[countNext] != input.charAt(countInput) && ++countNext < open.length);
                    if(countNext != open.length)
                        //next is open
                    {
                        chunks.push(input.charAt(countInput));
                    }
                    else
                        //next is close
                    {
                        countPrev = 0;
                        while(open[countPrev] != input.charAt(countInput) && ++countPrev < open.length);

                        countNext = 0;
                        while(close[countNext] != input.charAt(countInput) && ++countNext < close.length);

                        if(countNext == countPrev)
                            //closed a chunk.
                        {
                            chunks.pop();
                        }
                        else
                            //corrupted.
                        {
                            corrupt = true;
                            System.out.println(input.charAt(countInput));
                            switch(input.charAt(countInput))
                            {
                                case ')':
                                    sum += 3;
                                    break;
                                case ']':
                                    sum += 57;
                                    break;
                                case '}':
                                    sum += 1197;
                                    break;
                                case '>':
                                    sum += 25137;
                                    break;
                            }
                        }
                    }
                }
                else
                    // no active chunks;
                {
                    countNext = 0;
                    while(open[countNext] != input.charAt(countInput) && ++countNext < open.length);
                    if(countNext == open.length)
                        // corrupt. next one is a closer.
                    {
                        corrupt = true;
                        System.out.println(input.charAt(countInput));
                        switch(input.charAt(countInput))
                        {
                            case ')':
                                sum += 3;
                                break;
                            case ']':
                                sum += 57;
                                break;
                            case '}':
                                sum += 1197;
                                break;
                            case '>':
                                sum += 25137;
                                break;
                        }
                    }
                    else
                    {
                        chunks.push(input.charAt(countInput));
                    }
                }
            }
        }

        return sum;
    }

    public static int task2(String[] input)
    {
        return 0;
    }
}
