#include <falling_sand/sim/particles.hpp>

namespace falling_sand {
unsigned int getSquareColor(Particle particle) {
    switch (particle.state) {
        case EMPTY:
            return 0xFF000000;
        case SOLID:
            return 0xFF555555;
        case POWDER:
            return 0xFFFFFF00;

//        case SOLID:
//            return 0xFFfff700;
//        case SAND_EMITTER:
//            return 0xFFc7c600;
    }
}
}
