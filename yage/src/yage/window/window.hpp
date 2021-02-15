#ifndef FALLING_SAND_WINDOW_H
#define FALLING_SAND_WINDOW_H

#include <SDL.h>
#include <string>
#include "WindowConfig.h"

namespace yage::window {
    SDL_Window* create_window(WindowConfig &config);
};

#endif //FALLING_SAND_WINDOW_H
