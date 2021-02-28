#ifndef LIGHTBIKES_PLAYERINPUTPACKET_HPP
#define LIGHTBIKES_PLAYERINPUTPACKET_HPP

#include <lightbikes/input/InputState.hpp>

namespace lightbikes {
struct PlayerInputPacket {
    /// Frame the input is for
    int frame;
    // Player the input is for
    char player;
    /// ZE INPUT!!
    PlayerInputState playerInput;
};
}

#endif //LIGHTBIKES_PLAYERINPUTPACKET_HPP
