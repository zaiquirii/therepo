#include <SDL.h>
#include <yage/yage.h>
#include <yaml-cpp/yaml.h>
#include <falling_sand/sim/cell.hpp>
#include <falling_sand/ui/InputSystem.hpp>
#include <falling_sand/sim/SandboxConfig.hpp>
#include <falling_sand/sim/CellSystem.hpp>
#include <falling_sand/ui/Toolbox.hpp>

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
    Toolbox toolbox;

    while (!quit) {
        inputSystem.pollInput();
        if (inputSystem.quitRequested()) {
            quit = true;
        }

        if (!toolbox.takeInput(inputSystem)) {
            if (inputSystem.mouseDown()) {
                Point mousePos = inputSystem.mousePos(1280, 960, sim.width, sim.height);
                toolbox.currentBrush().paintAt(sim, mousePos);
            }
        }

        sim.tick();

        int size = sim.width * sim.height;
        Cell *currentState = sim.buffer();
        for (int i = 0; i < size; i++) {
            Cell s = currentState[i];
            pixels[i] = getCellColor(s);
        }

        SDL_UpdateTexture(texture, nullptr, pixels, config.width * sizeof(unsigned int));
        SDL_RenderCopy(renderer, texture, nullptr, nullptr);

        // RENDER TOOLBOX HERE
        toolbox.render(renderer);

        SDL_RenderPresent(renderer);

        SDL_Delay(16);
    }

    SDL_DestroyTexture(texture);
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}