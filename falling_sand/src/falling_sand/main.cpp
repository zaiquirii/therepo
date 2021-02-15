#include <SDL.h>
#include <yage/yage.hpp>
#include <yaml-cpp/yaml.h>
#include <falling_sand/sim/cell.hpp>
#include <falling_sand/ui/InputSystem.hpp>
#include <falling_sand/sim/SandboxConfig.hpp>
#include <falling_sand/sim/CellSim.hpp>
#include <falling_sand/ui/Toolbox.hpp>
#include "MainState.hpp"
#include "CellSystem.hpp"

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
    auto sim = CellSim(config.width, config.height);
    auto cellSystem = new CellSystem();

    Toolbox globalToolbox;

    SDL_Texture *texture = SDL_CreateTexture(
            renderer, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STATIC,
            config.width, config.height);
    auto *pixels = new unsigned int[config.width * config.height];
    memset(pixels, 0, config.width * config.height * sizeof(unsigned int));

    bool quit = false;
    unsigned char colorShift;

    yage::Game game = yage::Game();
    game.world().resources().set(&sim);
    game.world().resources().set(&globalToolbox);
    game.addSystem(cellSystem);
    game.setInitialState(new MainState(
            {1290, 960},
            {sim.width, sim.height},
            sim));

    int frameCount = 0;
    while (!quit) {
        game.nextFrame();

        int size = sim.width * sim.height;
        Cell *currentState = sim.buffer();
        for (int i = 0; i < size; i++) {
            Cell s = currentState[i];
            pixels[i] = getCellColor(s, colorShift);
        }

        SDL_UpdateTexture(texture, nullptr, pixels, config.width * sizeof(unsigned int));
        SDL_RenderCopy(renderer, texture, nullptr, nullptr);

        // RENDER TOOLBOX HERE
        game.world().resources().get<Toolbox>().render(renderer);

        SDL_RenderPresent(renderer);

        // TODO: Fix the framerate
        SDL_Delay(10);
        frameCount++;
        if (frameCount > 500) {
            quit = true;
        }
    }

    SDL_DestroyTexture(texture);
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}