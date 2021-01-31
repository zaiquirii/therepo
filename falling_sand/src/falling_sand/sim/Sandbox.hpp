#ifndef SRC_FALLING_SAND_SANDBOX_HPP
#define SRC_FALLING_SAND_SANDBOX_HPP

#include "particles.hpp"
#include "SandboxConfig.hpp"
#include "ParticleSystem.hpp"

namespace falling_sand {

class Sandbox {
public:
    Sandbox(SandboxConfig &config);

    void tick();

    Particle *currentState();

private:
    int width_;
    int height_;
    ParticleSystem system_;
};
}

#endif //SRC_FALLING_SAND_SANDBOX_HPP
