#ifndef SRC_FALLING_SAND_CELLSYSTEM_HPP
#define SRC_FALLING_SAND_CELLSYSTEM_HPP

#include <vector>
#include <falling_sand/ui/InputSystem.hpp>
#include "cell.hpp"

namespace falling_sand {
class CellSystem {
public:
    CellSystem(int width, int height);

    Cell *buffer() { return cells_; }

    Cell cellAt(int x, int y);

    void setCellAt(int x, int y, Cell cell);

    void tick();

    int width;
    int height;
private:
    void processCell(int index);

    Cell *cells_;
    int tickCount_;
    bool evenTick;
};

class CellAPI {
public:
    CellAPI(CellSystem *system, int x, int y, bool touchValue);

    Cell get(Point offset);

    void set(Point offset, Cell cell);
    int tickDirection() { return tickDirection_; }

private:
    CellSystem *system_;
    int tickDirection_;
    int x_;
    int y_;
};
}


#endif //SRC_FALLING_SAND_CELLSYSTEM_HPP
