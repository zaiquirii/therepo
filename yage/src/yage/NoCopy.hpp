#ifndef SRC_FALLING_SAND_NOCOPY_HPP
#define SRC_FALLING_SAND_NOCOPY_HPP

namespace yage {
class NoCopy {
protected:
    NoCopy() = default;
    NoCopy(NoCopy&) = delete;
    NoCopy(const NoCopy&) = delete;
    NoCopy& operator=(const NoCopy&) = delete;
    NoCopy& operator=(NoCopy&&) = delete;
};
}

#endif //SRC_FALLING_SAND_NOCOPY_HPP
