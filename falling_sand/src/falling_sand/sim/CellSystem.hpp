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
    bool touchValue;
};

class CellAPI {
public:
    CellAPI(CellSystem *system, int index);

    Cell get(Point offset);

    void set(Point offset, Cell particle);

private:
    CellSystem *system_;
    int index_;
    int x_;
    int y_;
};
}


#endif //SRC_FALLING_SAND_CELLSYSTEM_HPP
