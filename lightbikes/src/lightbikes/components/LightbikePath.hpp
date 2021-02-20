#ifndef LIGHTBIKES_LIGHTBIKEPATH_HPP
#define LIGHTBIKES_LIGHTBIKEPATH_HPP

#include <vector>
#include "Lightbike.hpp"
#include <SDL.h>

namespace lightbikes {
struct LightbikePath {
    Lightbike::Direction lastDirection;
    Uint8 color[3];
    std::vector<yage::Vec2> points;
};
}

#endif //LIGHTBIKES_LIGHTBIKEPATH_HPP
