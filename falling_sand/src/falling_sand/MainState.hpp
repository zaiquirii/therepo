#ifndef SRC_FALLING_SAND_MAINSTATE_HPP
#define SRC_FALLING_SAND_MAINSTATE_HPP


#include <yage/yage.hpp>
#include <falling_sand/ui/Toolbox.hpp>

namespace falling_sand {
class MainState : public yage::GameState {
public:
    void setup(yage::World &world) override;
    bool update(yage::World &world) override;

private:
    InputSystem inputSystem_;
    Point windowSize_;
};
}


#endif //SRC_FALLING_SAND_MAINSTATE_HPP
