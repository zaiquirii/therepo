#ifndef SRC_FALLING_SAND_BRUSH_HPP
#define SRC_FALLING_SAND_BRUSH_HPP

#include <falling_sand/sim/cell.hpp>
#include <falling_sand/sim/CellSystem.hpp>
#include "InputSystem.hpp"

namespace falling_sand {
enum BrushShape {
    Square
};
enum  BrushType {
    FillEmpty,
    Fill
};

struct Brush {
    Cell particle;
    BrushShape shape = Square;
    BrushType type = FillEmpty;
    int size;

    void paintAt(CellSystem &system, Point pos);
};
}

#endif //SRC_FALLING_SAND_BRUSH_HPP
