#include "BoundingBox.hpp"

namespace yage {
bool BoundingBox::intersects(const yage::BoundingBox &other) const {
    if (other.right < this->left || this->right < other.left) {
        return false;
    }

    if (other.top < this->bottom || this->top < other.bottom) {
        return false;
    }

    return true;
}
}
