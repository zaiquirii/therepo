#ifndef SRC_FALLING_SAND_PARTICLES_HPP
#define SRC_FALLING_SAND_PARTICLES_HPP

enum ParticleType: short {
    AIR = 0,
    WALL = 1,
    SAND = 2,
    SAND_EMITTER = 3,
};

namespace falling_sand {
struct Particle {
    ParticleType type;
};

unsigned int getSquareColor(Particle particle);

extern Particle borderParticle;
}

#endif //SRC_FALLING_SAND_PARTICLES_HPP
