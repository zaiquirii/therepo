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

void updateSand(Cell cell, CellAPI api) {
    Cell neighbor = api.get(offset(0, 1));
    if (neighbor.type == EMPTY) {
        api.set({}, neighbor);
        api.set(offset(0, 1), cell);
    } else {
        int dx = randomDirection();
        neighbor = api.get(offset(dx, 1));
        if (neighbor.type == EMPTY) {
            api.set({}, neighbor);
            api.set(offset(dx, 1), cell);
        }
    }
}

void updateWater(Cell cell, CellAPI api) {
    Cell neighbor = api.get(offset(0, 1));
    if (neighbor.type == EMPTY) {
        api.set({}, neighbor);
        api.set(offset(0, 1), cell);
        return;
    }

    int dx = randomDirection();
    neighbor = api.get(offset(dx, 1));
    if (neighbor.type == EMPTY) {
        api.set({}, neighbor);
        api.set(offset(dx, 1), cell);
        return;
    }

    dx = randomSide();
    neighbor = api.get(offset(dx, 0));
    if (neighbor.type == EMPTY) {
        api.set({}, neighbor);
        api.set(offset(dx, 0), cell);
        return;
    }
}
}
