#include <cstdlib>
#include "Sandbox.hpp"

namespace falling_sand {
static const Particle AIR = {.state = SOLID};
static const Particle WALL = {.state = SOLID};

void processParticle(ParticleSystem &system, int index);

Sandbox::Sandbox(SandboxConfig &config) : system_(config.width, config.height) {
    width_ = config.width;
    height_ = config.height;
}

Particle *Sandbox::currentState() {
    return system_.buffer();
}

void Sandbox::tick() {
    system_.startTick();
    int size = width_ * height_;
    for (int i = 0; i < size; i++) {
        processParticle(system_, i);
    }
}

void processParticle(ParticleSystem &system, int index) {
    int x = index % system.width;
    int y = index / system.width;
    Particle particle = system.particleAt(x, y);
    Particle otherParticle;
    switch (particle.state) {
        case SOLID:
        case EMPTY:
            system.setParticleAt(x, y, particle);
            break;
        case POWDER:
            if (y == 0) {
                system.setParticleAt(x, y, {.state = EMPTY});
                break;
            }

            otherParticle = system.particleAt(x, y -1);
            int z;
            switch (otherParticle.state) {
                case POWDER:
                    z = rand() % 2 ? -1 : 1;
                    otherParticle = system.particleAt(x + z, y - 1);
                    if (otherParticle.state == EMPTY) {
                        system.setParticleAt(x + z, y - 1, particle);
                        system.setParticleAt(x, y, otherParticle);
                        break;
                    }
                    otherParticle = system.particleAt(x - z, y - 1);
                    if (otherParticle.state == EMPTY) {
                        system.setParticleAt(x - z, y - 1, particle);
                        system.setParticleAt(x, y, otherParticle);
                        break;
                    }
                    // fall through
                case EMPTY:
                    system.setParticleAt(x, y, otherParticle);
                    system.setParticleAt(x, y - 1, particle);
                    break;
                case SOLID:
                    system.setParticleAt(x, y, particle);
                    break;
            }
            break;
    }
}
}
