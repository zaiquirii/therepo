#include "CellSystem.hpp"
#include <falling_sand/sim/CellSim.hpp>

namespace falling_sand {
void CellSystem::setup(yage::World &world) {
    auto &cellSim = world.resources().get<CellSim>();
    for (int i = 0; i < 25; i++) {
        cellSim.setCellAt(90 + i, 75, createCell(WALL));
        cellSim.setCellAt(90, 75 - i, createCell(WALL));
        cellSim.setCellAt(90 + 25, 75 - i, createCell(WALL));
        cellSim.setCellAt(90 + 25, 250 - i * 2, createCell(SAND));
    }
}

void CellSystem::fixedUpdate(yage::World &world) {
    auto &cellSim = world.resources().get<CellSim>();
    cellSim.tick();
}
};
