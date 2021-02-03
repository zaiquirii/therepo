//
// Created by Zachary Smith on 2/2/21.
//

#include "BitSet.hpp"

namespace falling_sand {
BitSet::BitSet(int size) {
    values_.resize(size);
    reset();
}

void BitSet::set(int index) {
    values_[index] = 1;
}

bool BitSet::isSet(int index) {
    return values_[index] != 0;
}

void BitSet::reset() {
    std::fill(values_.begin(), values_.end(), 0);
}
}
