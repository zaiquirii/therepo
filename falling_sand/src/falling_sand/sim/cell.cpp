#include <falling_sand/sim/cell.hpp>

namespace falling_sand {
Cell EMPTY_CELL = {.type = EMPTY};

unsigned int getSquareColor(Cell cell) {
    switch (cell.type) {
        case EMPTY:
            return 0xFF000000;
        case WALL:
            return 0xFF555555;
        case SAND:
            return 0xFFFFFF00;
        case WATER:
            return 0xFF0000FF;

//        case SOLID:
//            return 0xFFfff700;
//        case SAND_EMITTER:
//            return 0xFFc7c600;
    }
}
}
