//Using SDL and standard IO
#include <SDL.h>
#include <cstdio>
#include <filesystem>
#include <iostream>
#include <yaml-cpp/yaml.h>

#include "window/WindowConfig.h"

int main(int argc, char *args[]) {
    std::cout << "PATH: " << std::filesystem::current_path() << std::endl;
    auto windowConfig = YAML::LoadFile("../assets/config/window_config.yml").as<yage::window::WindowConfig>();

    //The window we'll be rendering to
    SDL_Window *window = nullptr;

    //The surface contained by the window
    SDL_Surface *screenSurface = nullptr;

    //Initialize SDL
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        printf("SDL could not initialize! SDL_Error: %s\n", SDL_GetError());
    } else {
        //Create window
        window = SDL_CreateWindow(windowConfig.title.c_str(),
                                  SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
                                  windowConfig.width, windowConfig.height, SDL_WINDOW_SHOWN);
        if (window == nullptr) {
            printf("Window could not be created! SDL_Error: %s\n", SDL_GetError());
        } else {
            //Get window surface
            screenSurface = SDL_GetWindowSurface(window);

            //Fill the surface white
            SDL_FillRect(screenSurface, nullptr, SDL_MapRGB(screenSurface->format, 0xFF, 0xFF, 0xFF));

            //Update the surface
            SDL_UpdateWindowSurface(window);

            bool quit = false;
            SDL_Event e;
            while (!quit) {
                while (SDL_PollEvent(&e)) {
                    if (e.type == SDL_QUIT) {
                        quit = true;
                    }
                }
            }
        }
    }

    //Destroy window
    SDL_DestroyWindow(window);

    //Quit SDL subsystems
    SDL_Quit();

    return 0;
}