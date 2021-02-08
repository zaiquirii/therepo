#include <falling_sand/sim/cell.hpp>
#include <cstdlib>

namespace falling_sand {

Cell EMPTY_CELL = {.type = EMPTY};

unsigned int darken(unsigned int r, unsigned int g, unsigned int b, float pct) {
    pct = 1 - pct;
    r = r * pct;
    g = g * pct;
    b = b * pct;

    return (0xFF << 24) | (r << 16) | (g << 8) | b;
}

Cell createCell(CellType type) {
    unsigned char color = rand() % 255;
    switch (type) {
        case EMPTY:
            return EMPTY_CELL;
        case SAND:
            return {.type = SAND, .color = color, .density = 52, .isSolid = true};
        case WALL:
            return {.type = WALL, .color = color, .isStatic = true};
        case WATER:
            return {.type = WATER, .color = color, .density = 50, .isLiquid = true};
        case OIL:
            return {.type = OIL, .color = color, .density = 48, .isLiquid = true, .flammability = 100};
        case WOOD:
            return {.type = WOOD, .color = color, .isStatic = true, .flammability = 10};
        case EMBER:
            return {.type = EMBER, .color = color, .isStatic = true, .valueA = 45};
        case FIRE:
            return {.type = FIRE, .color = color, .valueA = 20 + (rand() % 20)};
    }
    return Cell();
}

unsigned int getCellColor(Cell cell, unsigned char colorShift) {
    switch (cell.type) {
        case EMPTY:
            return 0xFF000000;
        case WALL:
            return 0xFF555555;
        case SAND:
            return darken(255, 255, 0, static_cast<float>(cell.color) / 255.0f * .2f);
        case WATER:
            return darken(0, 115, 255, static_cast<float>(cell.color + colorShift) / 255.0f * .3f);
        case OIL:
            return darken(0x85, 0x3F, 0, static_cast<float>(cell.color + colorShift) / 255.0f * .3f);
        case WOOD:
            return darken(0xac, 0x73, 0x39, static_cast<float>(cell.color) / 255.0f * .3f);
        case EMBER:
            return darken(255, 0, 0, static_cast<float>(cell.color + colorShift) / 255.0f * .3f);
        case FIRE:
            return darken(255, 0, 0, static_cast<float>(cell.color + colorShift) / 255.0f * .3f);
    }
}

}
