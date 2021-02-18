#include "InputSystem.hpp"

namespace lightbikes {
void InputSystem::setup(yage::World &world) {
    // Trying out setting resources inside of the system that "own" them.
    // This may not make sense for some resources.
    world.set<InputState>();
}

void InputSystem::fixedUpdate(yage::World &world) {
    while (SDL_PollEvent(&event_)) {
        switch (event_.type) {
            case SDL_QUIT:
                world.ctx<InputState>().quitRequested = true;
                break;
        }
    }
}
};
