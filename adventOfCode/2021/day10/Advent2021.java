import java.io.*;
import java.util.*;

public class Advent2021
{
    public static String[] fileInput(String FileName)
    {
        List<String>input = new LinkedList<String>();
        try
        {
            File myObj = new File(FileName);
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
