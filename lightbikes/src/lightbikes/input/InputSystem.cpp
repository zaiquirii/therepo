#include "InputSystem.hpp"

namespace lightbikes {
void InputSystem::setup(yage::World &world) {
    // Trying out setting resources inside of the system that "own" them.
    // This may not make sense for some resources.
    world.set<InputState>();
    auto &inputState = world.ctx<InputState>();
    for (int i = 0; i < 10; i++) {
        inputState.setPlayerInput(0, {Lightbike::Direction::Right});
        inputState.setPlayerInput(1, {Lightbike::Direction::Left});
    }
    // Window may not show until we consume these events
    while (SDL_PollEvent(&event_)){};
}

void InputSystem::fixedUpdate(yage::World &world) {
    auto &inputState = world.ctx<InputState>();
    auto oldPlayerOneState = inputState.getPlayerInput(0, 0);
    auto oldPlayerTwoState = inputState.getPlayerInput(1, 0);

    while (SDL_PollEvent(&event_)) {
        switch (event_.type) {
            case SDL_QUIT:
                world.ctx<InputState>().quitRequested = true;
                break;
            case SDL_KEYDOWN:
                switch (event_.key.keysym.sym) {
                    case SDLK_LEFT:
                        oldPlayerOneState = {Lightbike::Direction::Left};
                        break;
                    case SDLK_RIGHT:
                        oldPlayerOneState = {Lightbike::Direction::Right};
                        break;
                    case SDLK_UP:
                        oldPlayerOneState = {Lightbike::Direction::Up};
                        break;
                    case SDLK_DOWN:
                        oldPlayerOneState = {Lightbike::Direction::Down};
                        break;
                    case SDLK_a:
                        oldPlayerTwoState = {Lightbike::Direction::Left};
                        break;
                    case SDLK_d:
                        oldPlayerTwoState = {Lightbike::Direction::Right};
                        break;
                    case SDLK_w:
                        oldPlayerTwoState = {Lightbike::Direction::Up};
                        break;
                    case SDLK_s:
                        oldPlayerTwoState = {Lightbike::Direction::Down};
                        break;
                }
                break;
        }
    }
    inputState.setPlayerInput(0, oldPlayerOneState);
    inputState.setPlayerInput(1, oldPlayerTwoState);
}
};
