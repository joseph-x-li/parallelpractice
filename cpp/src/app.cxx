// Use C++ standard library
#include <iostream>
#include <string>

struct Test {
  int x;
  // Constructor
  Test(int x) : x(x) { std::cout<< "Constructor" << std::endl; }
  // Destructor
  ~Test() { std::cout<< "Destructor" << std::endl; }

  // Copy constructor
  Test(const Test& other) : x(other.x) { }
  // Copy assignment operator
  Test& operator=(const Test& other) {
    x = other.x;
    return *this;
  }

  // Move constructor
  Test(Test&& other) : x(other.x) { }
  // Move assignment operator
  Test& operator=(Test&& other) {
    x = other.x;
    return *this;
  }

  // Member function
  void print() {
    std::cout << x << std::endl;
  }
};

int main() {
  std::string answer = "abcdefghijklmnopqrstuvwxyz";
  std::cout << answer << std::endl;
  Test t(1);
}
