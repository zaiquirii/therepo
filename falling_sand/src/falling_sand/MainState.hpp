#ifndef SRC_FALLING_SAND_MAINSTATE_HPP
#define SRC_FALLING_SAND_MAINSTATE_HPP


#include <yage/yage.hpp>
#include <falling_sand/ui/Toolbox.hpp>

namespace falling_sand {
class MainState : public yage::GameState {
public:
    MainState(Point, Point, CellSim &cellSystem);
    bool update(yage::World &world) override;
private:
    // TODO: This is broken and incorrect, need a way to get resources going forward
    CellSim &cellSystem_;
    InputSystem inputSystem_;
    Point windowSize_;
    Point simSize_;
};
}


#endif //SRC_FALLING_SAND_MAINSTATE_HPP
