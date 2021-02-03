#include "cells.hpp"
#include <cstdlib>

namespace falling_sand {
Point offset(int x, int y) {
    return {.x = x, .y = y};
}

int randomDirection() {
    int value = rand() % 3;
    return 1 - value;
}

/// NO NEUTRAL
int randomSide() {
    return rand() % 2 == 0 ? -1 : 1;
}

bool processAsSolid(Cell &cell, CellAPI &api);

bool processAsLiquid(Cell &cell, CellAPI &api);

void updateSand(Cell cell, CellAPI api) {
    processAsSolid(cell, api);
}

void updateWater(Cell cell, CellAPI api) {
    processAsLiquid(cell, api);
}

void updateOil(Cell cell, CellAPI api) {
    processAsLiquid(cell, api);
}

bool processAsSolid(Cell &cell, CellAPI &api) {
    Cell neighbor = api.get(offset(0, 1));
    if (neighbor.type == EMPTY ||
        (neighbor.isLiquid &&
         neighbor.density < cell.density &&
         rand() % (cell.density - neighbor.density) != 0)) {
        api.set({}, neighbor);
        api.set(offset(0, 1), cell);
        return true;
    }

    int dx = randomDirection();
    neighbor = api.get(offset(dx, 1));
    if (neighbor.type == EMPTY ||
        (neighbor.isLiquid &&
         neighbor.density < cell.density &&
         rand() % (cell.density - neighbor.density) != 0)) {
        api.set({}, neighbor);
        api.set(offset(dx, 1), cell);
        return true;
    }
    return false;
}

bool processAsLiquid(Cell &cell, CellAPI &api) {
    Cell neighbor = api.get(offset(0, 1));
    if (neighbor.type == EMPTY ||
        (neighbor.isLiquid &&
         neighbor.density < cell.density &&
         rand() % (cell.density - neighbor.density) != 0)) {
        api.set({}, neighbor);
        api.set(offset(0, 1), cell);
        return true;
    }

    int dx = randomDirection();
    neighbor = api.get(offset(dx, 1));
    if (neighbor.type == EMPTY ||
        (neighbor.isLiquid &&
         neighbor.density < cell.density &&
         rand() % (cell.density - neighbor.density) != 0)) {
        api.set({}, neighbor);
        api.set(offset(dx, 1), cell);
        return true;
    }

    dx = randomSide();
    neighbor = api.get(offset(dx, 0));
    if (neighbor.type == EMPTY ||
        (neighbor.isLiquid &&
         neighbor.density < cell.density &&
         rand() % (cell.density - neighbor.density) != 0)) {
        api.set({}, neighbor);
        api.set(offset(dx, 0), cell);
        return true;
    }
    return false;
}
}
