#include "FrameRateLimiter.hpp"
#include <SDL.h>

namespace yage {
void FrameRateLimiter::delayFrame() {
    using namespace std::chrono;

    framesSeen_++;
    auto currentTime = Time::Clock::now();
    auto difference = duration_cast<Time::Seconds>(currentTime - lastCheckpoint_);
    // If we've been going for more than 1 second
    if (difference > Time::Seconds{1}) {
        lastFramerate_ = framesSeen_ / difference.count();
        lastCheckpoint_ = currentTime;
        framesSeen_ = 0;
//        printf("FRAMERATE: %f\n", lastFramerate_);
    }
    SDL_Delay(2);
}
}
