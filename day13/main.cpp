#include <iostream>
#include <sstream>
#include <fstream>

bool inOrder(const std::string& line_1, const std::string& line_2){
  
  return true;
}

int main()
{
  std::string input_path = "../input/day13/test_input.txt";

  // Get the input
  std::ifstream fs;
  fs.open(input_path, std::ios::in);

  if (!fs.is_open())
  {
    std::cerr << "unable to open file" << std::endl;
    return 1;
  }

  /** Part 1 */
  std::string line1,line2;
  int in_order_pairs = 0;

  while (std::getline(fs, line1, '\n')){
    if (line1.size() > 0){
      //get the second input
      std::getline(fs, line2, '\n');

      // Compare line 1 and 2
      if (inOrder(line1, line2)){
        in_order_pairs++;
      }
    }
    else{ //empty line
      
    }
    line1.clear();
    line2.clear();
  }

  std::cout << "in_order_pairs: " << in_order_pairs << "\n";

  return 0;
}