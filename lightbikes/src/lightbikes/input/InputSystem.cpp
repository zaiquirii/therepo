#include "InputSystem.hpp"

namespace lightbikes {
void InputSystem::setup(yage::World &world) {
    // Trying out setting resources inside of the system that "own" them.
    // This may not make sense for some resources.
    world.set<InputState>();
    auto &inputState = world.ctx<InputState>();
    inputState.setPlayerInput(0, {Lightbike::Direction::Right});
    inputState.setPlayerInput(1, {Lightbike::Direction::Left});

    // Window may not show until we consume these events
    while (SDL_PollEvent(&event_)){};
}

void InputSystem::fixedUpdate(yage::World &world) {
    auto &inputState = world.ctx<InputState>();
    while (SDL_PollEvent(&event_)) {
        switch (event_.type) {
            case SDL_QUIT:
                world.ctx<InputState>().quitRequested = true;
                break;
            case SDL_KEYDOWN:
                switch (event_.key.keysym.sym) {
                    case SDLK_LEFT:
                        inputState.setPlayerInput(0, {Lightbike::Direction::Left});
                        break;
                    case SDLK_RIGHT:
                        inputState.setPlayerInput(0, {Lightbike::Direction::Right});
                        break;
                    case SDLK_UP:
                        inputState.setPlayerInput(0, {Lightbike::Direction::Up});
                        break;
                    case SDLK_DOWN:
                        inputState.setPlayerInput(0, {Lightbike::Direction::Down});
                        break;
                    case SDLK_a:
                        inputState.setPlayerInput(1, {Lightbike::Direction::Left});
                        break;
                    case SDLK_d:
                        inputState.setPlayerInput(1, {Lightbike::Direction::Right});
                        break;
                    case SDLK_w:
                        inputState.setPlayerInput(1, {Lightbike::Direction::Up});
                        break;
                    case SDLK_s:
                        inputState.setPlayerInput(1, {Lightbike::Direction::Down});
                        break;
                }
                break;
        }
    }
}
};
