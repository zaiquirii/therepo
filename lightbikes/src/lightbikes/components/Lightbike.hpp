#ifndef LIGHTBIKES_LIGHTBIKE_HPP
#define LIGHTBIKES_LIGHTBIKE_HPP

namespace lightbikes {
struct Lightbike {
    enum Direction {
        None,
        Left,
        Right,
        Up,
        Down
    };

    float speed; // Units per second
    Direction direction;
    bool dead;
};
}

#endif //LIGHTBIKES_LIGHTBIKE_HPP
