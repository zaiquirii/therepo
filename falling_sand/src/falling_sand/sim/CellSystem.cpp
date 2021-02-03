#include "CellSystem.hpp"

#include <memory>
#include <falling_sand/sim/cells/cells.hpp>

namespace falling_sand {
CellSystem::CellSystem(int width, int height) {
    this->width = width;
    this->height = height;
    int size = width * height;

    cells_ = new Cell[size];
    memset(cells_, 0, size * sizeof(Cell));

    Cell wall = {.type = WALL};
    for (int i = 0; i < 25; i++) {
        setCellAt(90 + i, 75, wall);
        setCellAt(90, 75 - i, wall);
        setCellAt(90 + 25, 75 - i, wall);
    }

    for (int i = 0; i < width; i++) {
        setCellAt(i, height - 1, wall);
    }
}

Cell CellSystem::cellAt(int x, int y) {
    if (x >= 0 && x < width && y >= 0 && y < height) {
        int index = width * y + x;
        return cells_[index];
    }
    return { .type = WALL};
}

void CellSystem::setCellAt(int x, int y, Cell particle) {
    if (x >= 0 && x < width && y >= 0 && y < height) {
        int index = this->width * y + x;
        particle.touchValue = touchValue;
        cells_[index] = particle;
    }
}

void CellSystem::tick() {
    tickCount_++;
    touchValue = tickCount_ % 2 == 0;
    int size = width * height;
    for (int i = 0; i < size; i++) {
        processCell(i);
    }
}

void CellSystem::processCell(int index) {
    Cell cell = cells_[index];
    // This cell has already been touched this frame
    if (cell.touchValue == this->touchValue) {
        return;
    }

    CellAPI api(this, index);
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
    }
}

CellAPI::CellAPI(CellSystem *system, int index) :
        system_(system),
        index_(index),
        x_(index % system->width),
        y_(index / system->width) {}

Cell CellAPI::get(Point offset) {
    return system_->cellAt(x_ + offset.x, y_ + offset.y);
}

void CellAPI::set(Point offset, Cell particle) {
    return system_->setCellAt(x_ + offset.x, y_ + offset.y, particle);
}
}
