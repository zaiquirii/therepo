
#include "Sandbox.hpp"

namespace falling_sand {
inline int Sandbox::getIndex(int x, int y) const {
    return y * height_ + x;
}

Sandbox::Sandbox(SandboxConfig &config) {
    width_ = config.width;
    height_ = config.height;
    tickCount_ = 0;
    int size = width_ * height_;
    srcBuffer_ = new Square[size];
    dstBuffer_ = new Square[size];
    memset(srcBuffer_, 0, size * sizeof(Square));
    memset(dstBuffer_, 0, size * sizeof(Square));
}

void Sandbox::tick() {
//        for (int i = 0; i < 10; i++) {
//            srcBuffer_[tickCount_ + i] = Square::SAND;
//        }

    srcBuffer_[tickCount_] = Square::SAND;
//    std::swap(srcBuffer_, dstBuffer_);
    tickCount_ += 1;
}

Square *Sandbox::currentState() {
    return srcBuffer_;
}
}
