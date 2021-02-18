#ifndef SRC_FALLING_SAND_MATH_HPP
#define SRC_FALLING_SAND_MATH_HPP

#include <third_party/HandmadeMath.h>

namespace yage {
using Vec2 = hmm_v2;
using Vec3 = hmm_v3;
using Vec4 = hmm_v4;
using Mat4 = hmm_mat4;

inline Mat4 orthographic(float left, float right, float top, float bottom, float near, float far) {
    return HMM_Orthographic(left, right, top, bottom, near, far);
}

inline Mat4 scale(Vec3 scale) {
    return HMM_Scale(scale);
}

inline Mat4 translation(Vec3 translation) {
    return HMM_Translate(translation);
}
}

namespace YAML {
template<>
struct convert<yage::Vec2> {
    static bool decode(const Node &node, yage::Vec2 &rhs) {
        rhs.x = node[0].as<float>();
        rhs.y = node[1].as<float>();
        return true;
    }
};
}
#endif //SRC_FALLING_SAND_MATH_HPP

