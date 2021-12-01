import java.io.*;
import java.util.Scanner;

public class Depth
{
    public static void main(String[] args)
    {
        task1();
        task2();
    }

    public static void task1()
    {
        try
        {
            File myObj = new File("input1.txt");
            Scanner myReader = new Scanner(myObj);

            int currentData;
            int previousData;
            int increaseCount = 0;

            previousData = Integer.parseInt(myReader.nextLine());

            while(myReader.hasNextLine())
            {
                currentData = Integer.parseInt(myReader.nextLine());

                if(currentData > previousData)
                {
                    increaseCount++;
                }

                previousData = currentData;
            }

            System.out.println(increaseCount);

            myReader.close();
        }
        catch(FileNotFoundException e)
        {
            System.out.println("An Error Occurred: ");
            e.printStackTrace();
        }

    }

    public static void task2()
    {
        try
        {
            File myObj = new File("input1.txt");
            Scanner myReader = new Scanner(myObj);

            int a = 0, b = 0, c = 0, d = 0;
            int previous = Integer.MAX_VALUE, current;
            int count = 2;
            int increaseCount = 0;

            a = Integer.parseInt(myReader.nextLine());

            current = Integer.parseInt(myReader.nextLine());
            a += current;
            b += current;

            while(myReader.hasNextLine())
            {
                count++;
                current = Integer.parseInt(myReader.nextLine());
                switch(count % 4)
                {
                    case 0:
                        a += current;
                        c += current;
                        d += current;

                        if(c > previous)
                        {
                            increaseCount++;
                        }
                        previous = c;
                        c = 0;
                        break;
                    case 1:
                        a += current; 
                        b += current;
                        d += current;

                        if(d > previous)
                        {
                            increaseCount++;
                        }
                        previous = d;
                        d = 0;
                        break;
                    case 2:
                        a += current;
                        b += current;
                        c += current;

                        if(a > previous)
                        {
                            increaseCount++;
                        }
                        previous = a;
                        a = 0;
                        break;
                    case 3:
                        b += current;
                        c += current;
                        d += current;

                        if(b > previous)
                        {
                            increaseCount++;
                        }
                        previous = b;
                        b = 0;
                        break;
                }
            }

            System.out.println(increaseCount);

            myReader.close();
        }
        catch(FileNotFoundException e)
        {
            System.out.println("An Error Occurred: ");
            e.printStackTrace();
        }
    }
}
