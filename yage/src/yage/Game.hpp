#include "GameState.hpp"
#include "GameSystem.hpp"

#ifndef SRC_FALLING_SAND_GAME_HPP
#define SRC_FALLING_SAND_GAME_HPP

#include <memory>
#include <vector>
#include <SDL.h>

namespace yage {
class Game {
public:
    void addSystem(GameSystem *system);
    void setInitialState(GameState *state);
    void run();
    void nextFrame();
    World &world() { return world_; }

private:
    bool isRunning_;
    World world_;
    std::unique_ptr<GameState> currentState_;
    std::vector<std::unique_ptr<GameSystem>> systems_;
};
}

#endif //SRC_FALLING_SAND_GAME_HPP