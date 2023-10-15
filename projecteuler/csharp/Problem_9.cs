static void Main(string[] args)
{
    for (int a = 1; a < 1000; a++)
        for (int b = 1; b < 1000; b++)
            for (int c = 1; c < 1000; c++)
            {
                if (a * a + b * b != c * c)
                    continue;

                if (a + b + c == 1000)
                    Console.WriteLine("Found" +
                        ":\ta = " + a +
                        ";\tb = " + b +
                        ";\tc = " + c +
                        ";\ta * b * c = " + (a * b * c));
            }

    Console.WriteLine("All done :)");
    Console.ReadKey();
}
