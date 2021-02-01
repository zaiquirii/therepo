#ifndef SRC_FALLING_SAND_BRUSH_HPP
#define SRC_FALLING_SAND_BRUSH_HPP

#include <falling_sand/sim/particles.hpp>

namespace falling_sand {
enum BrushType {
    Square
};

struct Brush {
    Particle particle;
    BrushType type = Square;
    int size;
};
}

#endif //SRC_FALLING_SAND_BRUSH_HPP
