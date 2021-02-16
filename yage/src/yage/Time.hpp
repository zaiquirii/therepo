#ifndef SRC_FALLING_SAND_TIME_HPP
#define SRC_FALLING_SAND_TIME_HPP

#include <chrono>

namespace yage {
class Time {
public:
    using Clock = std::chrono::steady_clock;
    using Seconds = std::chrono::duration<double>;

    explicit Time(Seconds fixedDelta);

    void start();

    /*
     * Returns true is we have accumulated enough time for a fixed time step,
     * reduces accumulator by fixed time step
     */
    void accumulate();

    bool consumeFixedDelta();

    double fixedDelta() const { return fixedDeltaRaw_; }

    int fixedSteps() { return fixedStepCount_; }

private:
    Clock::time_point startTime_;
    Clock::time_point lastTimeSeen_;
    Seconds delta_;
    Seconds accumulatedTime_;
    Seconds fixedDelta_;
    double fixedDeltaRaw_;
    int fixedStepCount_;
};
}


#endif //SRC_FALLING_SAND_TIME_HPP
