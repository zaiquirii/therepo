#include <SDL.h>
#include <yage/yage.h>
#include <yaml-cpp/yaml.h>
#include <falling_sand/sim/Sandbox.hpp>
#include <falling_sand/sim/particles.hpp>

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
    auto sim = Sandbox(config);

    SDL_Texture *texture = SDL_CreateTexture(
            renderer, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STATIC,
            config.width, config.height);
    auto *pixels = new unsigned int[config.width * config.height];
    memset(pixels, 0, config.width * config.height * sizeof(unsigned int));

    bool quit = false;
    SDL_Event e;
    while (!quit) {
        sim.tick();

        int size = config.width * config.height;
        Particle *currentState = sim.currentState();
        for (int i = 0; i < size; i++) {
            Particle s = currentState[i];
            pixels[i] = getSquareColor(s);
        }

        SDL_UpdateTexture(texture, nullptr, pixels, config.width * sizeof(unsigned int));
        SDL_RenderCopy(renderer, texture, nullptr, nullptr);
        SDL_RenderPresent(renderer);

        while (SDL_PollEvent(&e)) {
            if (e.type == SDL_QUIT) {
                quit = true;
            }
        }
        SDL_Delay(100);
    }

    SDL_DestroyTexture(texture);
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}