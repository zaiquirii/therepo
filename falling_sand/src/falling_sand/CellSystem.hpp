#ifndef SRC_FALLING_SAND_CELLSYSTEM_HPP
#define SRC_FALLING_SAND_CELLSYSTEM_HPP


#include <yage/GameSystem.hpp>

namespace falling_sand {
class CellSystem: public yage::GameSystem {
public:
    bool setup(yage::World &world) override;
    void update(yage::World &world) override;
};
}


#endif //SRC_FALLING_SAND_CELLSYSTEM_HPP

