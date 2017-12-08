#include <stdio.h>
#include <vector>
#include <iostream>

int main() {
	auto v = std::vector<int>();
	char c;
	while (std::cin >> c)
		v.push_back(c - '0');
	
	v.push_back(v[0]);
	unsigned long long sum = 0;
	for (unsigned int i = v.size(); i-- != 1;)
		if (v[i-1] == v[i])
			sum += v[i];

	std::cout << sum << '\n';
	return 0;
}
