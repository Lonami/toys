#include <stdio.h>

/*
			..  <---58  57
	37  36  35  34  33  32  31  56
	38  17  16  15  14  13  30  55
	39  18   5   4   3  12  29  54
	40  19   6   1   2  11  28  53
	41  20   7   8   9  10  27  52
	42  21  22  23  24  25  26  51
	43  44  45  46  47  48  49  50


	-> at 1
	1 right -> at 2
	1 up, 2 left, 2 down, 2 right -> at 9
	1 right -> at 10
	3 up, 4 left, 4 down, 4 right -> at 25
	1 right -> at 26
	5 up, 6 left, 6 down, 6 right -> at 49
	1 right -> at 50
*/
int main() {
	int target;
	scanf("%d", &target);
	target--;

	int i = 0; // i
	int j = 0; // j
	int s = 0; // step
	int c = 0; // current
	int n = 2; // dimensions
	while (true)
	{
		// right
		j++;
		if (++c == target)
			goto done;

		// n-1 up
		for (s = n-1; s-- != 0;)
		{
			i--;
			if (++c == target)
				goto done;
		}
		// n left
		for (s = n; s-- != 0;)
		{
			j--;
			if (++c == target)
				goto done;
		}
		// n down
		for (s = n; s-- != 0;)
		{
			i++;
			if (++c == target)
				goto done;
		}
		// n right
		for (s = n; s-- != 0;)
		{
			j++;
			if (++c == target)
				goto done;
		}
		n += 2;
	}

	done:
	printf("reached %d at (%d, %d).\n", c+1, i, j);
	if (i < 0) i = -i;
	if (j < 0) j = -j;
	printf("solution is hence %d.\n", i + j);
	return 0;
}

