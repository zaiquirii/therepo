#include "cells.hpp"
#include <cstdlib>

namespace falling_sand {
int randomDirection() {
    int value = rand() % 3;
    return 1 - value;
}

/// NO NEUTRAL
int randomSide() {
    return rand() % 2 == 0 ? -1 : 1;
}

const float ACCELERATION = .15;
const float MAX_SPEED = 10;

bool canSwapPlaces(Cell &initiator, Cell &target) {
    return target.type == EMPTY ||
           (target.isLiquid &&
            initiator.density > target.density);
}

bool simulateFalling(Cell &cell, CellAPI &api) {
    int jump = static_cast<int>(cell.speed) + 1;
    int targetNeighbor = 0;
    Cell neighbor;
    for (int i = 1; i <= jump; i++) {
        neighbor = api.get({0, i});
        if (!canSwapPlaces(cell, neighbor)) {
            break;
        } else {
            targetNeighbor = i;
        }
    }

    if (targetNeighbor != 0) {
        neighbor = api.get({0, targetNeighbor});
        // Only continue accelerating if we didn't hit anything
        if (targetNeighbor == jump) {
            cell.speed += ACCELERATION;
        }
        float maxSpeed;
        if (neighbor.type == EMPTY) {
            maxSpeed = MAX_SPEED;
        } else {
            maxSpeed = 2;
        }

        if (cell.speed > maxSpeed) {
            cell.speed = maxSpeed;
        }

        api.set({0, targetNeighbor}, cell);
        api.set({}, neighbor);
        return true;
    }
    cell.speed = 0;
    return false;
}

bool processAsSolid(Cell &cell, CellAPI &api) {
    if (simulateFalling(cell, api)) {
        return true;
    }

    int dx = randomSide();
    Cell neighbor = api.get({dx, 1});
    if (canSwapPlaces(cell, neighbor)) {
        api.set({}, neighbor);
        api.set({dx, 1}, cell);
        return true;
    }
    return false;
}

bool processAsLiquid(Cell &cell, CellAPI &api) {
    cell.color -= rand() % 8;
    if (simulateFalling(cell, api)) {
        return true;
    }

    int dx = randomSide();
    Cell neighbor = api.get({dx, 1});
    if (canSwapPlaces(cell, neighbor)) {
        api.set({}, neighbor);
        api.set({dx, 1}, cell);
        return true;
    }

    neighbor = api.get({dx, 0});
    if (canSwapPlaces(cell, neighbor)) {
        api.set({}, neighbor);
        api.set({dx, 0}, cell);
        return true;
    }

    api.set({}, cell);

    // TODO: Do the particle collisions so we can have spread factor
//    dx *= 2;
//    neighbor = api.get({dx, 1});
//    if (canSwapPlaces(cell, neighbor)) {
//        api.set({}, neighbor);
//        api.set({dx, 1}, cell);
//        return true;
//    }
//
//    neighbor = api.get({dx, 0});
//    if (canSwapPlaces(cell, neighbor)) {
//        api.set({}, neighbor);
//        api.set({dx, 0}, cell);
//        return true;
//    }
//
//    dx /= 2;
//    dx *= 3;
//    neighbor = api.get({dx, 1});
//    if (canSwapPlaces(cell, neighbor)) {
//        api.set({}, neighbor);
//        api.set({dx, 1}, cell);
//        return true;
//    }
//
//    neighbor = api.get({dx, 0});
//    if (canSwapPlaces(cell, neighbor)) {
//        api.set({}, neighbor);
//        api.set({dx, 0}, cell);
//        return true;
//    }
    return false;
}

void updateSand(Cell cell, CellAPI api) {
    processAsSolid(cell, api);
}

void updateWater(Cell cell, CellAPI api) {
    processAsLiquid(cell, api);
}

void updateOil(Cell cell, CellAPI api) {
    processAsLiquid(cell, api);
}
}
