#ifndef SRC_FALLING_SAND_CELL_HPP
#define SRC_FALLING_SAND_CELL_HPP

namespace falling_sand {
enum CellType: short {
    EMPTY = 0,
    SAND = 1,
    WALL = 2,
    WATER = 3,
//    POWDER = 2,
//    LIQUID = 3,
//    GAS = 4
};

struct Cell {
    CellType type;
    bool touchValue;
};

extern Cell EMPTY_CELL;

unsigned int getSquareColor(Cell cell);
}

#endif //SRC_FALLING_SAND_CELL_HPP
