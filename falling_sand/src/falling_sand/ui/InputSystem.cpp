#include "InputSystem.hpp"
#include <iostream>

namespace falling_sand {
void InputSystem::pollInput() {
    while (SDL_PollEvent(&event_)) {
        switch (event_.type) {
            case SDL_QUIT:
                quitRequested_ = true;
            case SDL_MOUSEMOTION:
                rawMousePos_ = {.x = event_.motion.x, .y = event_.motion.y};
                break;
            case SDL_MOUSEBUTTONDOWN:
                if (event_.button.button == SDL_BUTTON_LEFT) {
                    mouseDown_ = true;
                }
                break;
            case SDL_MOUSEBUTTONUP:
                if (event_.button.button == SDL_BUTTON_LEFT) {
                    mouseDown_ = false;
                }
                break;
        }
    }
}

Point InputSystem::mousePos(int windowWidth, int windowHeight, int simWidth, int simHeight) {
    return {
            .x = static_cast<int>(rawMousePos_.x / ((float) windowWidth) * simWidth),
            .y = static_cast<int>(rawMousePos_.y / ((float) windowHeight) * simHeight)
    };
}
}
