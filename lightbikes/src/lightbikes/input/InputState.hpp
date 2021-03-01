#ifndef LIGHTBIKES_INPUTSTATE_HPP
#define LIGHTBIKES_INPUTSTATE_HPP

#include <lightbikes/components/Lightbike.hpp>
#include <lightbikes/utils/RingBuffer.hpp>
#include <vector>

namespace lightbikes {
struct PlayerInputState {
    Lightbike::Direction direction = Lightbike::Direction::None;
};

struct InputState {
    bool quitRequested = false;

    InputState();
    void setPlayerInput(int player, PlayerInputState inputState);

    [[nodiscard]] PlayerInputState getPlayerInput(int player, int frameOffset) const;

private:
    RingBuffer<PlayerInputState> playerInputs_[2];
};

}

#endif //LIGHTBIKES_INPUTSTATE_HPP
