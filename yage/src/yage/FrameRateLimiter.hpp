#ifndef SRC_FALLING_SAND_FRAMERATELIMITER_HPP
#define SRC_FALLING_SAND_FRAMERATELIMITER_HPP


#include "Time.hpp"

namespace yage {
// Currently this only calculates framerate

class FrameRateLimiter {
public:
    /// Delays frame to keep within max framerate
    void delayFrame();

    double framerate() { return lastFramerate_; }

private:
    int framesSeen_ = 0;
    Time::Clock::time_point lastCheckpoint_;
    double lastFramerate_ = 0;
};
}


#endif //SRC_FALLING_SAND_FRAMERATELIMITER_HPP
