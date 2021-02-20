#ifndef LIGHTBIKES_BOUNDINGBOX_HPP
#define LIGHTBIKES_BOUNDINGBOX_HPP

#include <yage/math.hpp>

namespace yage {
struct BoundingBox {
    union {
        struct {
            Vec2 topLeft;
            Vec2 bottomRight;
        };
        struct {
            float left;
            float top;
            float right;
            float bottom;
        };
    };

    [[nodiscard]] bool intersects(const BoundingBox &other) const;
};
}


#endif //LIGHTBIKES_BOUNDINGBOX_HPP
