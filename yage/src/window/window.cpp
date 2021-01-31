#include <SDL.h>
#include <yaml-cpp/yaml.h>
#include "WindowConfig.h"

namespace yage::window {
    SDL_Window *create_window(const char *path) {
        auto windowConfig = YAML::LoadFile(path).as<yage::window::WindowConfig>();
        SDL_Window *window = nullptr;
        if (SDL_Init(SDL_INIT_VIDEO) < 0) {
            printf("SDL could not initialize! SDL_Error: %s\n", SDL_GetError());
            return nullptr;
        }

        window = SDL_CreateWindow(windowConfig.title.c_str(),
                                  SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
                                  windowConfig.width, windowConfig.height, SDL_WINDOW_SHOWN);
        if (window == nullptr) {
            printf("SDL could not initialize! SDL_Error: %s\n", SDL_GetError());
            return nullptr;
        }
        return window;
    }
}