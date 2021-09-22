#ifndef LIGHTBIKES_GAMESTATEPACKET_HPP
#define LIGHTBIKES_GAMESTATEPACKET_HPP

#include <lightbikes/input/InputState.hpp>

namespace lightbikes {
struct BikeState {
    Lightbike::Direction direction;
};

struct GameStatePacket {
    /// Frame the input is for
    int frame;
    BikeState bikes[2];
};
}

#endif //LIGHTBIKES_GAMESTATEPACKET_HPP
