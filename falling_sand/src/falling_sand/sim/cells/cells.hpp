#ifndef SRC_FALLING_SAND_CELLS_HPP
#define SRC_FALLING_SAND_CELLS_HPP

#include <falling_sand/sim/cell.hpp>
#include <falling_sand/sim/CellSystem.hpp>

namespace falling_sand {
void updateSand(Cell cell, CellAPI api);

void updateWater(Cell cell, CellAPI api);

void updateOil(Cell cell, CellAPI api);

void updateEmber(Cell cell, CellAPI api);

void updateFire(Cell cell, CellAPI api);

void updateAcid(Cell cell, CellAPI api);

void updateIce(Cell cell, CellAPI api);

void updateLava(Cell cell, CellAPI api);

void updateStone(Cell cell, CellAPI api);
}

#endif //SRC_FALLING_SAND_CELLS_HPP
