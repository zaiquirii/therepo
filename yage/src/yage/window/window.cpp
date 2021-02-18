#include <SDL.h>
#include "../YageException.hpp"
#include "WindowConfig.h"

namespace yage::window {
SDL_Window *create_window(WindowConfig &windowConfig) {
    SDL_Window *window;
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        std::string error = "SDL could not initialize! SDL_Error: " + std::string(SDL_GetError());
        throw YageException(error);
    }

    window = SDL_CreateWindow(windowConfig.title.c_str(),
                              SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
                              windowConfig.width, windowConfig.height, SDL_WINDOW_SHOWN);
    if (window == nullptr) {
        SDL_Quit();
        std::string error = "SDL could not initialize! SDL_Error: " + std::string(SDL_GetError());
        throw YageException(error);
    }
    return window;
}
}