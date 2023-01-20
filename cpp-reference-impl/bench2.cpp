#include <iostream>

using namespace std;

int n, m;
int e[100100][2];

int main()
{
	cin.sync_with_stdio(false);

	cin >> n;
	cin >> m;
	for (int i = 0; i < m; i++)
	{
		cin >> e[i][0];
		cin >> e[i][1];
	}

	return 0;
}