#ifndef LIGHTBIKES_INPUTSTATE_HPP
#define LIGHTBIKES_INPUTSTATE_HPP

#include <lightbikes/components/Lightbike.hpp>

namespace lightbikes {
struct PlayerInputState {
    Lightbike::Direction direction = Lightbike::Direction::None;
};

struct InputState {
    bool quitRequested = false;
    std::vector<PlayerInputState> playerInputs_;
};
}

#endif //LIGHTBIKES_INPUTSTATE_HPP
