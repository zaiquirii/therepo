#include <cstdlib>
#include "Sandbox.hpp"

namespace falling_sand {
static const Particle airDefault = {.type = AIR};
static const Particle sandDefault = {.type = SAND};

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
    switch (particle.type) {
        case SAND:
            otherParticle = system.particleAt(x, y - 1);
            if (otherParticle.type == AIR) {
                system.setParticleAt(x, y - 1, sandDefault);
                system.setParticleAt(x, y, airDefault);
            } else if (otherParticle.type == SAND) {
                int z = rand() % 2 ? -1 : 1;
                if (system.particleAt(x + z, y - 1).type == AIR) {
                    system.setParticleAt(x + z, y - 1, sandDefault);
                    system.setParticleAt(x, y, airDefault);
                } else if (system.particleAt(x - z, y - 1).type == AIR) {
                    system.setParticleAt(x - z, y - 1, sandDefault);
                    system.setParticleAt(x, y, airDefault);
                } else {
                    system.setParticleAt(x, y, particle);
                }
            } else {
                system.setParticleAt(x, y, particle);
            }
            break;
        case SAND_EMITTER:
            system.setParticleAt(x, y, particle);
            if (system.particleAt(x, y - 1).type == AIR) {
                system.setParticleAt(x, y - 1, sandDefault);
            }
            break;
        case WALL:
        case AIR:
        default:
            system.setParticleAt(x, y, particle);
            break;
    }
}
}
