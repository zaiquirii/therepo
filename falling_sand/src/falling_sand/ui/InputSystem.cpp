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
            case SDL_KEYDOWN:
                switch (event_.key.keysym.sym) {
                    case SDLK_0:
                        keyPressed_ = 0;
                        break;
                    case SDLK_1:
                        keyPressed_ = 1;
                        break;
                    case SDLK_2:
                        keyPressed_ = 2;
                        break;
                    case SDLK_3:
                        keyPressed_ = 3;
                        break;
                    case SDLK_4:
                        keyPressed_ = 4;
                        break;
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
