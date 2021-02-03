#ifndef SRC_FALLING_SAND_CELLS_HPP
#define SRC_FALLING_SAND_CELLS_HPP

#include <falling_sand/sim/cell.hpp>
#include <falling_sand/sim/CellSystem.hpp>

namespace falling_sand {
    void updateSand(Cell cell, CellAPI api);
    void updateWater(Cell cell, CellAPI api);
}

#endif //SRC_FALLING_SAND_CELLS_HPP
