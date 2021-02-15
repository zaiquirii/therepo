#ifndef SRC_FALLING_SAND_CELLSIM_HPP
#define SRC_FALLING_SAND_CELLSIM_HPP

#include <yage/yage.hpp>
#include <vector>
#include <falling_sand/ui/InputSystem.hpp>
#include "cell.hpp"

namespace falling_sand {
class CellSim {
public:
    CellSim(int width, int height);

    void tick();

    Cell *buffer() { return cells_.get(); }

    Cell cellAt(int x, int y);

    void setCellAt(int x, int y, Cell cell);

    int width;
    int height;
private:
    void processCell(int index);

    std::unique_ptr<Cell> cells_;
    int tickCount_;
    bool evenTick;
};

class CellAPI {
public:
    CellAPI(CellSim &system, int x, int y, bool touchValue);

    Cell get(Point offset);

    void set(Point offset, Cell cell);

    int tickDirection() { return tickDirection_; }

private:
    CellSim &cellSim_;
    int tickDirection_;
    int x_;
    int y_;
};
}


#endif //SRC_FALLING_SAND_CELLSIM_HPP
