#ifndef LIGHTBIKES_INPUTSYSTEM_HPP
#define LIGHTBIKES_INPUTSYSTEM_HPP


#include <yage/yage.hpp>
#include <SDL.h>
#include "InputState.hpp"

namespace lightbikes {
class InputSystem : public yage::GameSystem {
    void setup(yage::World &world) override;

    void fixedUpdate(yage::World &world) override;

private:
    SDL_Event event_;
};
}


#endif //LIGHTBIKES_INPUTSYSTEM_HPP
