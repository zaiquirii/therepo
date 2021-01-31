#ifndef SRC_FALLING_SAND_PARTICLESYSTEM_HPP
#define SRC_FALLING_SAND_PARTICLESYSTEM_HPP

#include "particles.hpp"

namespace falling_sand {
class ParticleSystem {
public:
    ParticleSystem(int width, int height);

    Particle *buffer() { return outputBuffer_; }

    Particle particleAt(int x, int y);
    void setParticleAt(int x, int y, Particle particle);
    void startTick();

    int width;
    int height;
private:
    Particle *inputBuffer_;
    Particle *outputBuffer_;
};
}


#endif //SRC_FALLING_SAND_PARTICLESYSTEM_HPP
