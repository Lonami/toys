static void Main(string[] args)
{
    Console.WriteLine("Searching...");

    int cPrimeCount = 2;
    for (int i = 3; i < int.MaxValue; i += 2)
        if (isPrime(i) && cPrimeCount++ == 10001)
            Console.WriteLine(i);

    Console.ReadKey();
}

static bool isPrime(int number)
{
    if (number < 4) return true;

    for (int i = 2; i < number; i++)
        if (number % i == 0)
            return false;

    return true;
}
