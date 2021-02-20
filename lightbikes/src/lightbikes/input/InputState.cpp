#include "InputState.hpp"

namespace lightbikes {
void InputState::setPlayerInput(int player, PlayerInputState inputState) {
    playerInputs_[player] = inputState;
}
}
