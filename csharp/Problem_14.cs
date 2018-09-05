using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace ProjectEulerSolver
{
    class Program
    {
        static void Main(string[] args)
        {
            int longest = 0;
            const int maxNumber = 1000000;
            for (int i = 1; i < maxNumber; i++)
            {
                var collatz = CountCollatz(i);
                if (collatz > longest)
                {
                    longest = collatz;
                    Console.WriteLine($"[{((float)i / (float)maxNumber).ToString("p")}]\tNew longest chain of {longest}\tdigits for {i}");
                }
            }

            Console.WriteLine("No longer chains!");
            Console.ReadKey();
        }

        static void PrintCollatz(long n)
        {
            while (n != 1)
            {
                Console.Write($"{n} -> ");

                if (n % 2 == 0)
                {
                    n = n / 2;
                }
                else
                {
                    n = 3 * n + 1;
                }
            }

            Console.WriteLine("1");
        }

        static int CountCollatz(long n)
        {
            int count = 1;
            while (n != 1)
            {
                if (n % 2 == 0)
                {
                    n = n / 2;
                }
                else
                {
                    n = 3 * n + 1;
                }

                ++count;
            }

            return count;
        }
    }
}
