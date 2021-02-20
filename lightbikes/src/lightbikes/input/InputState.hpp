#ifndef LIGHTBIKES_INPUTSTATE_HPP
#define LIGHTBIKES_INPUTSTATE_HPP

#include <lightbikes/components/Lightbike.hpp>
#include <vector>

namespace lightbikes {
struct PlayerInputState {
    Lightbike::Direction direction = Lightbike::Direction::None;
};

struct InputState {
    bool quitRequested = false;

    void setPlayerInput(int player, PlayerInputState inputState);
    PlayerInputState getPlayerInput(int player) { return playerInputs_[player]; }

private:
    std::vector<PlayerInputState> playerInputs_ = std::vector<PlayerInputState>(2);
};

}

#endif //LIGHTBIKES_INPUTSTATE_HPP
