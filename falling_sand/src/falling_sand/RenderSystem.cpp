#include <yage/yage.hpp>
#include <falling_sand/SandboxConfig.hpp>
#include <falling_sand/sim/CellSim.hpp>
#include <falling_sand/ui/Toolbox.hpp>
#include <yage/window/WindowConfig.h>
#include "RenderSystem.hpp"

namespace falling_sand {
void RenderSystem::setup(yage::World &world) {
    auto &config = world.resources().get<FallingSandConfig>();
    window_ = yage::window::create_window(config.window);
    renderer_ = SDL_CreateRenderer(window_, -1, 0);
    texture_ = SDL_CreateTexture(
            renderer_, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STATIC,
            config.sandbox.width, config.sandbox.height);
    pixels_ = new unsigned int[config.sandbox.width * config.sandbox.height];
    memset(pixels_, 0, config.sandbox.width * config.sandbox.height * sizeof(unsigned int));
}

void RenderSystem::update(yage::World &world) {
    auto &cellSim = world.resources().get<CellSim>();
    int size = cellSim.width * cellSim.height;
    Cell *currentState = cellSim.buffer();
    for (int i = 0; i < size; i++) {
        Cell s = currentState[i];
        pixels_[i] = getCellColor(s, 0);
    }

    SDL_UpdateTexture(texture_, nullptr, pixels_, cellSim.width * sizeof(unsigned int));
    SDL_RenderCopy(renderer_, texture_, nullptr, nullptr);

    // RENDER TOOLBOX HERE
    world.resources().get<Toolbox>().render(renderer_);
    SDL_RenderPresent(renderer_);
}

void RenderSystem::tearDown(yage::World &world) {
    SDL_DestroyTexture(texture_);
    SDL_DestroyRenderer(renderer_);
    SDL_DestroyWindow(window_);
    SDL_Quit();
}
}
