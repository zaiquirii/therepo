#include <falling_sand/sim/cell.hpp>
#include <cstdlib>
#include <cstdio>

namespace falling_sand {

Cell EMPTY_CELL = {.type = EMPTY};
Cell WALL_CELL = {.type = WALL, .isStatic = true};
Cell SAND_CELL = {.type = SAND, .density = 52, .isSolid = true};
Cell WATER_CELL = {.type = WATER, .density = 50, .isLiquid = true};
Cell OIL_CELL = {.type = OIL, .density = 48, .isLiquid = true};

unsigned int darken(unsigned int r, unsigned int g, unsigned int b, float pct) {
    pct = 1 - pct;
    r = r * pct;
    g = g * pct;
    b = b * pct;

    return (0xFF<<24) | (r << 16) | (g << 8) | b;

}

unsigned int getCellColor(Cell cell) {
    switch (cell.type) {
        case EMPTY:
            return 0xFF000000;
        case WALL:
            return 0xFF555555;
        case SAND:
            return darken(255, 255, 0, static_cast<float>(cell.color) / 255.0f * .2f);
        case WATER:
            return darken(0, 115, 255, static_cast<float>(cell.color) / 255.0f * .3f);
//            return darken(0, 115, 255, (rand() % 255) / 255.0 * .3f);
        case OIL:
            return 0xFF853F00;
    }
}
}
