//
// Created by Zachary Smith on 2/15/21.
//

#ifndef SRC_FALLING_SAND_YAGEEXCEPTION_HPP
#define SRC_FALLING_SAND_YAGEEXCEPTION_HPP

#include <exception>
#include <string>

namespace yage {
class YageException: public std::exception {
public:
    explicit YageException(std::string error);
    const char * what() const noexcept override;
private:
    std::string error_;
};
}


#endif //SRC_FALLING_SAND_YAGEEXCEPTION_HPP
