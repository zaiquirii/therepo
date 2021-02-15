#import "Game.hpp"

namespace yage {
void Game::addSystem(GameSystem *system) {
    systems_.push_back(std::unique_ptr<GameSystem>(system));
}

void Game::run() {
    for (auto &system: systems_) {
        system->setup(world_);
    }

    // Game loop here
    while (isRunning_) {
        nextFrame();
    }
}

void Game::setInitialState(GameState *state) {
    currentState_ = std::unique_ptr<GameState>(state);
}

void Game::nextFrame() {
    isRunning_ = currentState_->update(world_);
    for (auto &system: systems_) {
        system->update(world_);
    }
}
}
