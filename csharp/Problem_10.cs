static void Main(string[] args)
{
    calcPrimes(2000000);
    Console.WriteLine(primes.Count + " primes found");

    long sum = 0;
    foreach (var prime in primes)
        sum += prime;

    Console.WriteLine("Sum of all of them: " + sum);
    Console.ReadKey();
}

static List<int> primes;
static void calcPrimes(int until)
{
    primes = new List<int>();
    // add the only even prime
    primes.Add(2);
    for (int i = 3; i < until; i += 2)
    {
        bool success = true;
        foreach (var prime in primes)
            if (i % prime == 0)
            {
                success = false;
                break;
            }

        if (success)
            primes.Add(i);

        if (i % 2001 == 0)
            Console.Write("\rCalculating primes... " + (((float)i / (float)until)).ToString("0.00%"));
    }
    Console.WriteLine();
}
