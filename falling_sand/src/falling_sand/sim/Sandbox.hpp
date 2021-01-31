#ifndef SRC_FALLING_SAND_SANDBOX_HPP
#define SRC_FALLING_SAND_SANDBOX_HPP

#include <memory>
#include "squares.hpp"
#include "SandboxConfig.hpp"

namespace falling_sand {
    class Sandbox {
    public:
        Sandbox(SandboxConfig &config);

        void tick();
        Square *currentState();

    private:
        int getIndex(int x, int y) const;

        int tickCount_;
        int width_;
        int height_;
        Square *srcBuffer_;
        Square *dstBuffer_;
    };
}

#endif //SRC_FALLING_SAND_SANDBOX_HPP
