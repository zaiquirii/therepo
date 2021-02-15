#include "Brush.hpp"

namespace falling_sand {
bool shouldPaint(Cell cell, BrushType type) {
    if (type == Fill) {
        return true;
    }
    return cell.type == EMPTY;
}

void Brush::paintAt(CellSim &system, Point pos) {
    int startY = pos.y - size / 2;
    int startX = pos.x - size / 2;

    float yPct = 0;
    float xPct = 0;
    for (int i = 0; i < size; i++) {
        yPct = static_cast<float>(i) / static_cast<float>(size) * 2 - 1;
        xPct = 1 - sqrt(1 - yPct * yPct);
        int offset = floor(xPct * static_cast<float>(size) / 2);

        for (int j = offset; j < size - offset; j++) {
            int x = startX + j;
            int y = startY + i;
            if (x >= 0 && x < system.width &&
                y >= 0 && y < system.height &&
                shouldPaint(system.cellAt(x, y), type)) {
                Cell newCell = particle;
                newCell.color = rand();
                system.setCellAt(x, y, newCell);
            }
        }
    }
}
}
