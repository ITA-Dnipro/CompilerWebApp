#include <fstream>
#include <iostream>
#include <exception>
/*int main(int argc, char* argv[]) 
{
    std::cout << "Create new file. " << "Filename: " << argv[1] << std::endl; 
    std::ofstream output(argv[1]);
}*/

int main() 
{ 
    try {
        auto filename = "test/data/new_file_created_with_so";
        std::cout << "main(). Create new file. " << "Filename: " << filename << std::endl; 
        std::ofstream output(filename);
        output << "Hello from shared object" << std::endl;
        output.close();
    } catch (const std::exception& e) {
        return 0;
    }
}
