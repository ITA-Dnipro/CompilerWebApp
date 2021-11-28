#include <fstream>
using namespace std;

int main() {
    auto file = ofstream("test/data/file_with_content.txt");
    file << "Content from shared object!";
}