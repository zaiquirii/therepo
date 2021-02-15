//
// Created by Zachary Smith on 2/15/21.
//

#include "YageException.hpp"

namespace yage {
YageException::YageException(std::string error) :
        error_(std::move(error)),
        std::exception() {}

const char *YageException::what() const noexcept {
    return error_.c_str();
}
}
