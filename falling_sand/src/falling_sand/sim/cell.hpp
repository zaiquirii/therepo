#ifndef SRC_FALLING_SAND_CELL_HPP
#define SRC_FALLING_SAND_CELL_HPP

namespace falling_sand {
enum CellType : short {
    EMPTY = 0,
    SAND = 1,
    WALL = 2,
    WATER = 3,
    OIL = 4,
    WOOD = 5,
    EMBER = 6,
    FIRE = 7,
};

struct Cell {
    CellType type;
    unsigned char color;
    short density;
    bool touched;
    bool isLiquid;
    bool isSolid;
    bool isStatic;
    char flammability;
    float speedX;
    float speedY;
    int valueA;
};

extern Cell EMPTY_CELL;

Cell createCell(CellType type);

unsigned int getCellColor(Cell cell, unsigned char colorShift);
}

#endif //SRC_FALLING_SAND_CELL_HPP
