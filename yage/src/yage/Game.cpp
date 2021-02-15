#import "Game.hpp"

namespace yage {
void Game::addSystem(GameSystem *system) {
    systems_.push_back(std::unique_ptr<GameSystem>(system));
}

void Game::run() {
    assert(currentState_ != nullptr);
    currentState_->setup(world_);
    for (auto &system: systems_) {
        system->setup(world_);
    }

    // Game loop here
    isRunning_ = true;
    while (isRunning_) {
        nextFrame();
    }

    for (auto &system: systems_) {
        system->tearDown(world_);
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
