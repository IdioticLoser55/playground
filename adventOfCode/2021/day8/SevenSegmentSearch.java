import java.util.*;
import java.util.stream.Stream;
import java.io.*;

public class SevenSegmentSearch
{
    public static void main(String[] args)
    {
        String[] input = Advent2021.fileInput("input.txt");
        System.out.println("Task 1: " + task1(input));
        System.out.println("Task 2: " + task2(input));
    }
    public static int task1(String[] input)
    {
        String[] entry; 
        int one = 0, four = 0, seven = 0, eight = 0;

        for(int i = 0; i < input.length; i++)
        {
            entry = input[i].split("( \\| )| ");
            switch(entry[10].length())
            {
                case 2:
                    one ++;
                    break;
                case 3:
                    seven ++;
                    break;
                case 4:
                    four ++;
                    break;
                case 7:
                    eight ++;
                break;
            }
            switch(entry[11].length())
            {
                case 2:
                    one ++;
                    break;
                case 3:
                    seven ++;
                    break;
                case 4:
                    four ++;
                    break;
                case 7:
                    eight ++;
                break;
            }
            switch(entry[12].length())
            {
                case 2:
                    one ++;
                    break;
                case 3:
                    seven ++;
                    break;
                case 4:
                    four ++;
                    break;
                case 7:
                    eight ++;
                break;
            }
            switch(entry[13].length())
            {
                case 2:
                    one ++;
                    break;
                case 3:
                    seven ++;
                    break;
                case 4:
                    four ++;
                    break;
                case 7:
                    eight ++;
                break;
            }
        }

        return one + four + seven + eight;
    }

    public static int task2(String[] input)
    {
        int count = 0;
        int count2 = 0;
        int sum = 0;
        String[] entry;
        String[] encoding = new String[10];
        
        for(int i = 0; i < input.length; i++)
        {
            entry = input[i].split("( \\| )| ");

            //match 1
            count = -1;
            while(entry[++count].length() != 2);
            encoding[1] = entry[count];
            //match 7
            count = -1;
            while(entry[++count].length() != 3);
            encoding[7] = entry[count];
            //match 4
            count = -1;
            while(entry[++count].length() != 4);
            encoding[4] = entry[count];
            //match 8
            count = -1;
            while(entry[++count].length() != 7);
            encoding[8] = entry[count];

            count = -1;
            while(++count < 10 )
            {
                if(!entry[count].equals(encoding[1]) && !entry[count].equals(encoding[4]) && !entry[count].equals(encoding[7]) && !entry[count].equals(encoding[8]) )
                {
                    count2 = 0;
                    for(int j = 0; j < encoding[1].length(); j++)
                    {
                        if(entry[count].contains(Character.toString(encoding[1].charAt(j))))
                        {
                            count2++;
                        }
                    }

                    if(count2 == 2)
                        //must be 0, 3 or 9
                    {
                        if(entry[count].length() == 5)
                        {
                            encoding[3] = entry[count];
                        }
                        else
                            //0, 9
                        {
                            count2 = 0;
                            for(int j = 0; j < encoding[4].length(); j++)
                            {
                                if(entry[count].contains(Character.toString(encoding[4].charAt(j))))
                                {
                                    count2++;
                                }
                            }
                            if(count2 == 4)
                                //9
                            {
                                encoding[9] = entry[count];
                            }
                            else
                                //0
                            {
                                encoding[0] = entry[count];
                            }
                        }

                    }
                    else
                        // 2, 5, 6
                    {
                        if(entry[count].length() == 6)
                            //6
                        {
                            encoding[6] = entry[count];
                        }
                        else
                            //2, 5
                        {
                            count2 = 0;
                            for(int j = 0; j < encoding[4].length(); j++)
                            {
                                if(entry[count].contains(Character.toString(encoding[4].charAt(j))))
                                {
                                    count2++;
                                }
                            }


                            if(count2 == 3)
                                //5
                            {
                                encoding[5] = entry[count];
                            }
                            else
                                //2
                            {
                                encoding[2] = entry[count];
                            }
                        }
                    }
                }
            }

//            for(int j = 0; j < encoding.length; j++)
//            {
//                System.out.println(encoding[j]);
//            }
//            System.out.println();
//            for(int j = 0; j < entry.length; j++)
//            {
//                System.out.println(entry[j]);
//            }

            entry[0] = "";
            for(int j = 10; j < entry.length; j++)
            {
                count = -1;
                count2 = 0;
                boolean match = false;
                while(!match)
                {
                    count++;
                    match = true;
                    if(encoding[count].length() == entry[j].length())
                    {
                        count2 = 0;
                        while(match && count2 < encoding[count].length())
                        {
                            if(!entry[j].contains(Character.toString(encoding[count].charAt(count2))))
                            {
                                match = false;
                            }
                            count2++;
                        }
                    }
                    else
                    {
                        match = false;
                    }
                }

                entry[0] = entry[0] + Integer.toString(count);
            }

            sum += Integer.parseInt(entry[0]);
        }

        return sum;
    }
}
