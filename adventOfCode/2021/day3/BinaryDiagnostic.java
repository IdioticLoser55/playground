import java.io.*;
import java.util.*;

public class BinaryDiagnostic
{
    public static void main(String[] args)
    { 
        String[] input = fileInput("input1.txt");
        System.out.println("Task1: " + task1(input));
        System.out.println("Task2: " + task2(input));
    }

    public static int task1(String[] input)
    {
        int gamma = 0, gammaCount;
        int epsilon = 0, epsilonCount;
        int binary = 1 << input[0].length();

        for(int i = 0; i < input[0].length(); i++)
        {
            binary >>= 1;
            gammaCount = 0;
            epsilonCount = 0;

            for(int j = 0; j < input.length; j++)
            {
                if(input[j].charAt(i) == '1')
                {
                    gammaCount++;
                }
                else
                {
                    epsilonCount++;
                }
            }

            gammaCount = gammaCount > epsilonCount ? 1 : 0;
            epsilonCount = gammaCount == 1 ? 0 : 1;

            gamma += gammaCount * binary;
            epsilon += epsilonCount * binary;
        }

        return gamma * epsilon;
    }

    public static int task2(String[] input)
    {
        int ogr, co2;
        int oneCount = 0, zeroCount = 0, currentIndex = 0;
        List<String> reports = new LinkedList<String>();
        List<String> remove = new LinkedList<String>();
        for(String report : input) { reports.add(report);}

        while(reports.size() > 1)
        {
            for(String report : reports)
            {
                if(report.charAt(currentIndex) == '1')
                {
                    oneCount++;
                }
                else
                {
                    zeroCount++;
                }
            }

            oneCount = oneCount >= zeroCount ? 1 : 0;
            for(String report : reports)
            {
                if(Integer.parseInt(Character.toString(report.charAt(currentIndex))) == oneCount)
                {
                    remove.add(report);
                }
            }
            reports.removeAll(remove);

            remove.clear(); 
            oneCount = 0; zeroCount = 0;
            currentIndex++;
        }

        oneCount = 1 << reports.get(0).length();
        for(int i = 0; i < reports.get(0).length(); i++)
        {
            oneCount >>= 1;

            if(reports.get(0).charAt(i) == '1')
            {
                zeroCount += oneCount;
            }
        }

        ogr = zeroCount;

        currentIndex = 0;
        oneCount = 0; zeroCount = 0;
        reports.clear();
        for(String report : input) { reports.add(report);}

        while(reports.size() > 1)
        {
            for(String report : reports)
            {
                if(report.charAt(currentIndex) == '1')
                {
                    oneCount++;
                }
                else
                {
                    zeroCount++;
                }
            }

            oneCount = zeroCount > oneCount ? 1 : 0;
            for(String report : reports)
            {
                if(Integer.parseInt(Character.toString(report.charAt(currentIndex))) == oneCount)
                {
                    remove.add(report);
                }
            }
            reports.removeAll(remove);

            remove.clear(); 
            oneCount = 0; zeroCount = 0;
            currentIndex++;
        }

        oneCount = 1 << reports.get(0).length();
        for(int i = 0; i < reports.get(0).length(); i++)
        {
            oneCount >>= 1;

            if(reports.get(0).charAt(i) == '1')
            {
                zeroCount += oneCount;
            }
        }

        co2 = zeroCount;


        return co2 * ogr;
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
