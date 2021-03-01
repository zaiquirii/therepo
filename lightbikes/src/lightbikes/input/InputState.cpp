#include "InputState.hpp"

namespace lightbikes {
InputState::InputState() : playerInputs_{RingBuffer<PlayerInputState>(10), RingBuffer<PlayerInputState>(10)} {
}

void InputState::setPlayerInput(int player, PlayerInputState inputState) {
    playerInputs_[player].push(inputState);
}

PlayerInputState InputState::getPlayerInput(int player, int frameOffset) const {
    return playerInputs_[player].get(frameOffset);
}
}
