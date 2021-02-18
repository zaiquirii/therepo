#ifndef LIGHTBIKES_POSITION_HPP
#define LIGHTBIKES_POSITION_HPP

#include <yage/yage.hpp>

namespace lightbikes {
struct Position {
    float x;
    float y;

    [[nodiscard]] yage::Vec4 asVec() const {
        return {x, y};
    }
};
}

#endif //LIGHTBIKES_POSITION_HPP
