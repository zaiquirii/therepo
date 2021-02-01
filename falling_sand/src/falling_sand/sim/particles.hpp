#ifndef SRC_FALLING_SAND_PARTICLES_HPP
#define SRC_FALLING_SAND_PARTICLES_HPP

enum ParticleState: short {
    EMPTY = 0,
    SOLID = 1,
    POWDER = 2,
//    LIQUID = 3,
//    GAS = 4
};

namespace falling_sand {
struct Particle {
    ParticleState state;
    int weight;
};

unsigned int getSquareColor(Particle particle);
}

#endif //SRC_FALLING_SAND_PARTICLES_HPP
