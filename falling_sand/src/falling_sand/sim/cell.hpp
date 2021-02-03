#ifndef SRC_FALLING_SAND_CELL_HPP
#define SRC_FALLING_SAND_CELL_HPP

namespace falling_sand {
enum CellType : short {
    EMPTY = 0,
    SAND = 1,
    WALL = 2,
    WATER = 3,
    OIL = 4,
//    POWDER = 2,
//    LIQUID = 3,
//    GAS = 4
};

struct Cell {
    CellType type;
    short density = 0;
    bool touchValue;
    bool isLiquid;
    bool isSolid;
    bool isStatic;
};

extern Cell EMPTY_CELL;
extern Cell SAND_CELL;
extern Cell WALL_CELL;
extern Cell WATER_CELL;
extern Cell OIL_CELL;

unsigned int getSquareColor(Cell cell);
}

#endif //SRC_FALLING_SAND_CELL_HPP
