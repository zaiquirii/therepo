#include "CellSystem.hpp"

#include <memory>
#include <falling_sand/sim/cells/cells.hpp>

namespace falling_sand {
CellSystem::CellSystem(int width, int height) {
    this->width = width;
    this->height = height;
    int size = width * height;

    cells_ = new Cell[size];
    for (int i = 0; i < size; i++) {
        cells_[i] = EMPTY_CELL;
    }

    for (int i = 0; i < 25; i++) {
        setCellAt(90 + i, 75, createCell(WALL));
        setCellAt(90, 75 - i, createCell(WALL));
        setCellAt(90 + 25, 75 - i, createCell(WALL));
        setCellAt(90 + 25, 250 - i * 2, createCell(SAND));
    }
}

Cell CellSystem::cellAt(int x, int y) {
    if (x >= 0 && x < width && y >= 0 && y < height) {
        int index = width * y + x;
        return cells_[index];
    }
    return {.type = WALL};
}

void CellSystem::setCellAt(int x, int y, Cell cell) {
    if (x >= 0 && x < width && y >= 0 && y < height) {
        int index = this->width * y + x;
        cell.touched = true;
        cells_[index] = cell;
    }
}

void CellSystem::tick() {
    tickCount_++;
    evenTick = tickCount_ % 2 == 0;
    int size = width * height;
    for (int i = 0; i < size; i++) {
        cells_[i].touched = false;
    }

    for (int i = 0; i < size; i++) {
        if (evenTick) {
            processCell(i);
        } else {
            processCell(size - i - 1);
        }
    }
}

void CellSystem::processCell(int index) {
    Cell cell = cells_[index];
    // This cell has already been touched this frame
    if (cell.touched) {
        return;
    }

    int x = index % width;
    int y = index / width;

    CellAPI api(this, x, y, evenTick);
    switch (cell.type) {
        case SAND:
            updateSand(cell, api);
            break;
        case WATER:
            updateWater(cell, api);
            break;
        case OIL:
            updateOil(cell, api);
            break;
        case EMBER:
            updateEmber(cell, api);
            break;
        case FIRE:
            updateFire(cell, api);
            break;
    }
}

CellAPI::CellAPI(CellSystem *system, int x, int y, bool touchValue) :
        system_(system),
        x_(x),
        y_(y),
        tickDirection_(touchValue ? -1 : 1) {}

Cell CellAPI::get(Point offset) {
    return system_->cellAt(x_ + offset.x, y_ + offset.y);
}

void CellAPI::set(Point offset, Cell cell) {
    return system_->setCellAt(x_ + offset.x, y_ + offset.y, cell);
}
}
