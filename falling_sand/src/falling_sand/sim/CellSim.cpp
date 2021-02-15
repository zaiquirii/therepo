#include "CellSim.hpp"

#include <memory>
#include <falling_sand/sim/cells/cells.hpp>

namespace falling_sand {
CellSim::CellSim(int width, int height) :
        width(width), height(height),
        cells_(std::unique_ptr<Cell>(new Cell[width * height])) {}

Cell CellSim::cellAt(int x, int y) {
    if (x >= 0 && x < width && y >= 0 && y < height) {
        int index = width * y + x;
        return cells_.get()[index];
    }
    return {.type = WALL};
}

void CellSim::setCellAt(int x, int y, Cell cell) {
    if (x >= 0 && x < width && y >= 0 && y < height) {
        int index = this->width * y + x;
        cell.touched = true;
        cells_.get()[index] = cell;
    }
}

void CellSim::tick() {
    tickCount_++;
    evenTick = tickCount_ % 2 == 0;
    int size = width * height;
    for (int i = 0; i < size; i++) {
        cells_.get()[i].touched = false;
    }

    for (int i = 0; i < size; i++) {
        if (evenTick) {
            processCell(i);
        } else {
            processCell(size - i - 1);
        }
    }
}

void CellSim::processCell(int index) {
    Cell cell = cells_.get()[index];
    // This cell has already been touched this frame
    if (cell.touched) {
        return;
    }

    int x = index % width;
    int y = index / width;

    CellAPI api(*this, x, y, evenTick);
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
        case ACID:
            updateAcid(cell, api);
            break;
        case ICE:
            updateIce(cell, api);
            break;
        case LAVA:
            updateLava(cell, api);
            break;
        case STONE:
            updateStone(cell, api);
            break;
    }
}

CellAPI::CellAPI(CellSim &system, int x, int y, bool touchValue) :
        cellSim_(system),
        x_(x),
        y_(y),
        tickDirection_(touchValue ? -1 : 1) {}

Cell CellAPI::get(Point offset) {
    return cellSim_.cellAt(x_ + offset.x, y_ + offset.y);
}

void CellAPI::set(Point offset, Cell cell) {
    return cellSim_.setCellAt(x_ + offset.x, y_ + offset.y, cell);
}
}
