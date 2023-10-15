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
        static List<int> primes = new List<int>(500);

        static void Main(string[] args)
        {
            /*
New max: 4      divisors for 10
New max: 16     divisors for 120
New max: 18     divisors for 300
New max: 24     divisors for 630
New max: 40     divisors for 3240
New max: 48     divisors for 5460
New max: 90     divisors for 25200
New max: 112    divisors for 73920
New max: 128    divisors for 157080
New max: 144    divisors for 437580
New max: 162    divisors for 749700
New max: 168    divisors for 1385280
New max: 240    divisors for 2031120
New max: 320    divisors for 2162160
New max: 480    divisors for 17907120
New max: 576    divisors for 76576500
            */
            
            var max = 0;
            int triangle = 0;
            for (int i = 1; true; i += 1)
            {
                triangle += i;
                if (triangle % 10 != 0) // any number with a lot of divisors probably is multiple of 10, if it's not, skip
                    continue;

                var divisors = GetDivisors(triangle);
                if (divisors > max)
                {
                    max = divisors;
                    Console.WriteLine($"\rNew max: {max}\tdivisors for {triangle}");
                }
            }
        }


        static int GetDivisors(int value)
        {
            int divisorCount = 0;

            for (int i = 1; i <= value; i++)
                if (value % i == 0)
                    ++divisorCount;

            return divisorCount;
        }
    }
}
