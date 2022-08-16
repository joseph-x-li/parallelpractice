#include <string>
#include <vector>
// #include <cstdint>

class AlphabetUtil {

  private:
    // Array of times for each letter
    std::vector<float> times;

  public:
    // String containing the "answer" 
    std::string answer;

  
    // Constructor
    AlphabetUtil(std::string answer) {
      this->answer = answer;
      this->times = std::vector<float>(answer.length());
    }

    // Destructor
    ~AlphabetUtil() { }

    void setAnswer(std::string answer) {this->answer = answer;}
    const std::string& getAnswer() { &answer;}



};