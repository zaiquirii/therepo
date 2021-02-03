#ifndef SRC_FALLING_SAND_BITSET_HPP
#define SRC_FALLING_SAND_BITSET_HPP

#include <vector>

namespace falling_sand {
class BitSet {
public:
    BitSet(int size);

    void set(int index);

    bool isSet(int index);

    void reset();

private:
    std::vector<unsigned char> values_;
};
}


#endif //SRC_FALLING_SAND_BITSET_HPP
