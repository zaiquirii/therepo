#include "Time.hpp"

namespace yage {

Time::Time(Time::Seconds fixedDelta)
        : fixedDelta_(fixedDelta),
          fixedDeltaRaw_(fixedDelta.count()) {}

void Time::start() {
    lastTimeSeen_ = Clock::now();
    startTime_ = lastTimeSeen_;
}

void Time::accumulate() {
    auto currentTime = Clock::now();
    auto difference = currentTime - lastTimeSeen_;
    delta_ = Seconds(difference);
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
