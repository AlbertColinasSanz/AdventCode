#include <iostream>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>
#include <fstream>
#include <string>


using namespace std;

int main(int argc, char const *argv[])
{
    std::cout << "argc == " << argc << '\n';
    
    std::fstream my_file;
    std::noskipws;

	my_file.open("data/info.txt", ios::in);
    
    if (!my_file) {
		cout << "File not created!";
	} else {
        char ch;
        string file_info = string();
        
		while (my_file >> file_info);

        cout << file_info;

        my_file.close();
    }

    

    
    return 0;
}
