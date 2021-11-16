#include <cstdio>
#include <iostream>

using namespace std;

int main() {
    auto file_name = "test/data/testfile";
    cout << "Delete file: " << file_name << endl;
    remove(file_name);
}