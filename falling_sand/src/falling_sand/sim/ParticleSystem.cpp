#include "ParticleSystem.hpp"
#include <memory>

namespace falling_sand {
ParticleSystem::ParticleSystem(int width, int height) {
    this->width = width;
    this->height = height;
    int size = width * height;
    inputBuffer_ = new Particle[size];
    outputBuffer_ = new Particle[size];
    memset(inputBuffer_, 0, size * sizeof(Particle));
    memset(outputBuffer_, 0, size * sizeof(Particle));

    setParticleAt(100, 149, { .type = SAND_EMITTER});
    setParticleAt(150, 149, { .type = SAND_EMITTER});
    setParticleAt(149, 149, { .type = SAND_EMITTER});
    setParticleAt(148, 149, { .type = SAND_EMITTER});
    setParticleAt(0, 149, { .type = SAND_EMITTER});

    for (int i = 0; i < 25; i++) {
        setParticleAt(90 + i, 75, { .type = WALL });
        setParticleAt(90, 75 + i, { .type = WALL });
        setParticleAt(90+25, 75 + i, { .type = WALL });
    }
}

Particle ParticleSystem::particleAt(int x, int y) {
    if (x < 0 || x >= this->width || y < 0 || y >= this->height) {
        return borderParticle;
    }
    return inputBuffer_[this->width * (this->height - y - 1) + x];
}

void ParticleSystem::setParticleAt(int x, int y, Particle particle) {
    outputBuffer_[this->width * (this->height - 1 - y) + x] = particle;
}

void ParticleSystem::startTick() {
    std::swap(inputBuffer_, outputBuffer_);
}
}
