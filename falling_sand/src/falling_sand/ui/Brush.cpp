#include "Brush.hpp"

namespace falling_sand {
bool shouldPaint(Cell cell, BrushType type) {
    if (type == Fill) {
        return true;
    }
    return cell.type == EMPTY;
}

void Brush::paintAt(CellSystem &system, Point pos) {
    int startY = pos.y - size / 2;
    int startX = pos.x - size / 2;

    for (int i = 0; i < size; i++) {
        for (int j = 0; j < size; j++) {
            int x = startX + j;
            int y = startY + i;
            if (x >= 0 && x < system.width &&
                y >= 0 && y < system.height &&
                shouldPaint(system.cellAt(x, y), type)) {
                system.setCellAt(x, y, particle);
            }
        }
    }
}
}
