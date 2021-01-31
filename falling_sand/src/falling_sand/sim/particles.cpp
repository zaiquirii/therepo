#include <falling_sand/sim/particles.hpp>

namespace falling_sand {
Particle borderParticle = {.type = WALL};

unsigned int getSquareColor(Particle particle) {
    switch (particle.type) {
        case AIR:
            return 0xFF000000;
        case SAND:
            return 0xFFfff700;
        case SAND_EMITTER:
            return 0xFFc7c600;
        case WALL:
            return 0xFF555555;
    }
}
}
