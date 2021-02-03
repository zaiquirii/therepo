#include <SDL.h>
#include <yage/yage.h>
#include <yaml-cpp/yaml.h>
#include <falling_sand/sim/cell.hpp>
#include <falling_sand/ui/InputSystem.hpp>
#include <falling_sand/ui/Brush.hpp>
#include <falling_sand/sim/SandboxConfig.hpp>
#include <falling_sand/sim/CellSystem.hpp>

using namespace falling_sand;

int main(int argc, char *args[]) {
    SDL_Window *window = yage::window::create_window("../assets/config/window_config.yml");
    if (window == nullptr) {
        SDL_DestroyWindow(window);
        SDL_Quit();
        return 1;
    }

    SDL_Renderer *renderer = SDL_CreateRenderer(window, -1, 0);
    auto config = YAML::LoadFile("../assets/config/sandbox_config.yml").as<SandboxConfig>();
    auto sim = CellSystem(config.width, config.height);

    SDL_Texture *texture = SDL_CreateTexture(
            renderer, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STATIC,
            config.width, config.height);
    auto *pixels = new unsigned int[config.width * config.height];
    memset(pixels, 0, config.width * config.height * sizeof(unsigned int));

    bool quit = false;
    InputSystem inputSystem;
    SDL_Event e;
    Brush brush = {.particle = SAND_CELL, .size = 10};

    while (!quit) {
        inputSystem.pollInput();
        if (inputSystem.quitRequested()) {
            quit = true;
        }

        switch (inputSystem.keyPressed()) {
            case 0:
                brush.particle = EMPTY_CELL;
                brush.type = Fill;
                break;
            case 1:
                brush.particle = WALL_CELL;
                brush.type = FillEmpty;
                break;
            case 2:
                brush.particle = SAND_CELL;
                brush.type = FillEmpty;
                break;
            case 3:
                brush.particle = WATER_CELL;
                brush.type = FillEmpty;
                break;
            case 4:
                brush.particle = OIL_CELL;
                brush.type = FillEmpty;
                break;

        }

        if (inputSystem.mouseDown()) {
            Point mousePos = inputSystem.mousePos(1280, 960, sim.width, sim.height);
            brush.paintAt(sim, mousePos);
        }

        sim.tick();

        int size = sim.width * sim.height;
        Cell *currentState = sim.buffer();
        for (int i = 0; i < size; i++) {
            Cell s = currentState[i];
            pixels[i] = getSquareColor(s);
        }

        SDL_UpdateTexture(texture, nullptr, pixels, config.width * sizeof(unsigned int));
        SDL_RenderCopy(renderer, texture, nullptr, nullptr);
        SDL_RenderPresent(renderer);

        SDL_Delay(5);
    }

    SDL_DestroyTexture(texture);
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}