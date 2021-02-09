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

Point randomNeighbor() {
    return {randomDirection(), randomDirection()};
}

const float ACCELERATION = .15;
const float MAX_SPEED = 10;

bool canSwapPlaces(Cell &initiator, Cell &target) {
    return target.type == EMPTY ||
           (target.isLiquid &&
            initiator.density > target.density);
}

bool simulateFalling(Cell &cell, CellAPI &api) {
    int jump = static_cast<int>(cell.speedX) + 1;
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
            cell.speedX += ACCELERATION;
        }
        float maxSpeed;
        if (neighbor.type == EMPTY) {
            maxSpeed = MAX_SPEED;
        } else {
            maxSpeed = 2;
        }

        if (cell.speedX > maxSpeed) {
            cell.speedX = maxSpeed;
        }

        api.set({0, targetNeighbor}, cell);
        api.set({}, neighbor);
        return true;
    }
    cell.speedX = 0;
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

    int ranDir;
    if (cell.speedY != 0) {
        ranDir = cell.speedY > 0 ? 1 : -1;
    } else {
        ranDir = randomSide();
    }

    Cell neighbor;
    Point targetOffset = {0, 0};
    int spread = 3;
    for (int i = 1; i <= spread; i++) {
        int delta = i * ranDir;
        neighbor = api.get({delta, 1});
        if (canSwapPlaces(cell, neighbor)) {
            targetOffset = {delta, 1};
            break;
        } else {
            neighbor = api.get({delta, 0});
            if (!canSwapPlaces(cell, neighbor)) {
                break;
            } else {
                targetOffset = {delta, 0};
            }
        }
    }

    // The x would have to have been changed if we found a target
    if (targetOffset.x != 0) {
        cell.speedY = ranDir;
        neighbor = api.get(targetOffset);
        api.set({}, neighbor);
        api.set(targetOffset, cell);
        return true;
    }

    cell.speedY = 0;
    api.set({}, cell);
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

void updateEmber(Cell cell, CellAPI api) {
    // valueA for ember is a lifetime parameter
    Point offset = randomNeighbor();
    Cell neighbor = api.get(offset);
    if (neighbor.flammability > 0 &&
        (rand() % 100) < neighbor.flammability) {
        api.set(offset, createCell(EMBER));
    } else if (neighbor.type == EMPTY) {
        api.set(offset, createCell(FIRE));
    }

    cell.valueA--;
    if (cell.valueA == 0) {
        api.set({}, EMPTY_CELL);
    } else {
        cell.color -= rand() % 8;
        api.set({}, cell);
    }
}

void updateFire(Cell cell, CellAPI api) {
    // valueA for fire is a lifetime parameter
    cell.valueA--;
    if (cell.valueA == 0) {
        api.set({}, EMPTY_CELL);
        return;
    }

    cell.color -= rand() % 8;
    Point offset = {randomDirection(), -1};
    Cell neighbor = api.get(offset);
    if (neighbor.type == EMPTY) {
        api.set({}, neighbor);
        api.set(offset, cell);
    } else {
        // Remove fire
        if (neighbor.type == ICE) {
            api.set(offset, createCell(WATER));
        } else if (neighbor.flammability > 0 &&
                   (rand() % 100) < neighbor.flammability) {
            api.set(offset, createCell(EMBER));
        }
        api.set({}, EMPTY_CELL);
    }
}

void updateAcid(Cell cell, CellAPI api) {
    Point offset = {randomDirection(), rand() % 2};
    Cell neighbor = api.get(offset);
    if (neighbor.type != EMPTY &&
        neighbor.type != ACID &&
        neighbor.type != WALL) {
        api.set({}, EMPTY_CELL);
        api.set(offset, EMPTY_CELL);
    } else {
        processAsLiquid(cell, api);
    }
}

void updateIce(Cell cell, CellAPI api) {
    Point offset = randomNeighbor();
    Cell neighbor = api.get(offset);
    if (neighbor.type == WATER && rand() % 4 == 0) {
        api.set(offset, createCell(ICE));
    }
}

void updateLava(Cell cell, CellAPI api) {
    cell.color -= rand() % 8;
    if (processAsLiquid(cell, api)) {
        return;
    } else {
        api.set({}, cell);
    }

    Point offset = randomNeighbor();
    Cell neighbor = api.get(offset);
    if (neighbor.flammability > 0 &&
        (rand() % 100) < neighbor.flammability) {
        api.set(offset, createCell(EMBER));
    }
}

void updateStone(Cell cell, CellAPI api) {
    Cell upLeft = api.get({-1, -1});
    Cell upRight = api.get({1, -1});
    if (upLeft.type != STONE || upRight.type != STONE) {
        simulateFalling(cell, api);
    } else {
        cell.speedY = 0;
    }
}
}
