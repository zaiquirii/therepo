#include <cstdio>
#include "Time.hpp"

namespace yage {

Time::Time(Time::Duration fixedDelta) : fixedDelta_(fixedDelta) {
    fixedDeltaRaw_ = fixedDelta.count();
    printf("fixed delta: %f\n", fixedDeltaRaw_);
}

void Time::start() {
    using namespace std::chrono;
    lastTimeSeen_ = steady_clock::now();
    startTime_ = lastTimeSeen_;
}

void Time::accumulate() {
    using namespace std::chrono;
    auto currentTime = steady_clock::now();
    auto difference = currentTime - lastTimeSeen_;
    delta_ = Duration(difference);
    accumulatedTime_ += delta_;
    lastTimeSeen_ = currentTime;
}

bool Time::consumeFixedDelta() {
    if (accumulatedTime_ > fixedDelta_) {
        accumulatedTime_ -= fixedDelta_;
        fixedStepCount_++;
        return true;
    }
    return false;
}
}
