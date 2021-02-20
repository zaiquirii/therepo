#include <iostream>
#import "Game.hpp"
#include "Time.hpp"
#include "FrameRateLimiter.hpp"

namespace yage {
const static auto DEFAULT_FIXED_TIME = Time::Seconds{1} / 60.0; // 60 fps

void Game::addSystem(GameSystem *system) {
    systems_.push_back(std::unique_ptr<GameSystem>(system));
}

void Game::run() {
    assert(currentState_ != nullptr);

    currentState_->setup(world_);
    for (auto &system: systems_) {
        system->setup(world_);
    }

    FrameRateLimiter frameRateLimiter;
    // Make time available to all things
    // Intentionally not making it available in setup for now
    world_.set<Time>(DEFAULT_FIXED_TIME);
    world_.ctx<Time>().start();

    std::cout << "running game loop" << std::endl;
    // Game loop here
    bool isRunning = true;
    while (isRunning) {
        auto &time = world_.ctx<Time>();
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

        frameRateLimiter.delayFrame();
    }

    for (auto &system: systems_) {
        system->tearDown(world_);
    }
}

void Game::setInitialState(GameState *state) {
    currentState_ = std::unique_ptr<GameState>(state);
}
}
