#ifndef FALLING_SAND_WINDOW_H
#define FALLING_SAND_WINDOW_H

#include <SDL.h>
#include <string>

namespace yage::window {
    SDL_Window* create_window(const char* path);
};

#endif //FALLING_SAND_WINDOW_H
