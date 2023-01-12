#include <iostream>
#include <string>
#include <sstream>
#include <vector>
#include <tuple>
 
int main() {

std::vector<std::string> v0;
std::string v1; std::getline(std::cin, v1);
std::istringstream v3(v1); std::string v2;
while (std::getline(v3, v2, ' ')) { v0.push_back(v2); }
int v4 = v0.size();
int n;
std::istringstream v5(v0[0]);
v5 >> n;
std::vector<std::tuple<int>> d;
for (int i=0; i<n; i++) {
std::vector<std::string> v6;
std::string v7; std::getline(std::cin, v7);
std::istringstream v9(v7); std::string v8;
while (std::getline(v9, v8, ' ')) { v6.push_back(v8); }
int v10 = v6.size();
int v12;
std::istringstream v13(v6[0]);
v13 >> v12;
auto v11 = std::make_tuple(v12);
d.push_back(v11);
}


 
    return 0;
}