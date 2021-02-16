#import "Game.hpp"
#include "Time.hpp"

namespace yage {
const static auto DEFAULT_FIXED_TIME = Time::Duration{1} / 60.0; // 60 fps

void Game::addSystem(GameSystem *system) {
    systems_.push_back(std::unique_ptr<GameSystem>(system));
}

void Game::run() {
    assert(currentState_ != nullptr);

    currentState_->setup(world_);
    for (auto &system: systems_) {
        system->setup(world_);
    }

    Time time(DEFAULT_FIXED_TIME);
    // Make time available to all things
    // Intentionally not making it available in setup for now
    world_.resources().set(&time);
    time.start();

    // Game loop here
    bool isRunning = true;
    while (isRunning) {
        time.accumulate();
        while (time.consumeFixedDelta()) {
            isRunning = currentState_->fixedUpdate(world_);
            for (auto &system: systems_) {
                system->fixedUpdate(world_);
            }
        }

        isRunning = isRunning && currentState_->update(world_);
        for (auto &system: systems_) {
            system->update(world_);
        }
    }

    for (auto &system: systems_) {
        system->tearDown(world_);
    }
}

void Game::setInitialState(GameState *state) {
    currentState_ = std::unique_ptr<GameState>(state);
}
}
