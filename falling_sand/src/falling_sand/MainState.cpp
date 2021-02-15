//
// Created by Zachary Smith on 2/13/21.
//

#include "MainState.hpp"

namespace falling_sand {
MainState::MainState(Point windowSize, Point simSize, CellSim &cellSystem) :
        windowSize_(windowSize), simSize_(simSize), cellSystem_(cellSystem) {}

bool MainState::update(yage::World &world) {
    auto &toolbox = world.resources().get<Toolbox>();

    inputSystem_.pollInput();
    if (!toolbox.takeInput(inputSystem_)) {
        Point mousePos = inputSystem_.mousePos(windowSize_.x, windowSize_.y, simSize_.x, simSize_.y);
        toolbox.currentBrush().paintAt(cellSystem_, mousePos);
    }
    return inputSystem_.quitRequested();
}
}
