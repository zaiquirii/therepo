#include "MainState.hpp"
#include "SandboxConfig.hpp"

namespace falling_sand {
void MainState::setup(yage::World &world) {
    auto &config = world.resources().get<FallingSandConfig>();
    windowSize_ = {config.window.width, config.window.height};
}

bool MainState::fixedUpdate(yage::World &world) {
    auto &toolbox = world.resources().get<Toolbox>();
    auto &cellSim = world.resources().get<CellSim>();

    inputSystem_.pollInput();
    if (!toolbox.takeInput(inputSystem_)) {
        if (inputSystem_.mouseDown()) {
            Point mousePos = inputSystem_.mousePos(windowSize_.x, windowSize_.y, cellSim.width, cellSim.height);
            toolbox.currentBrush().paintAt(cellSim, mousePos);
        }
    }
    return !inputSystem_.quitRequested();
}
}
