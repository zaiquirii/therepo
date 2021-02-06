#include <falling_sand/sim/cell.hpp>

namespace falling_sand {

Cell EMPTY_CELL = {.type = EMPTY};
Cell WALL_CELL = {.type = WALL, .isStatic = true};
Cell SAND_CELL = {.type = SAND, .density = 52, .isSolid = true};
Cell WATER_CELL = {.type = WATER, .density = 50, .isLiquid = true};
Cell OIL_CELL = {.type = OIL, .density = 48, .isLiquid = true};

unsigned int getCellColor(Cell cell) {
    switch (cell.type) {
        case EMPTY:
            return 0xFF000000;
        case WALL:
            return 0xFF555555;
        case SAND:
            return 0xFFFFFF00;
        case WATER:
            return 0xFF0000FF;
        case OIL:
            return 0xFF853F00;
    }
}
}
