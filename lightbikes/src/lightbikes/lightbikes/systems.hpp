#ifndef LIGHTBIKES_SYSTEMS_HPP
#define LIGHTBIKES_SYSTEMS_HPP

#include <yage/yage.hpp>

namespace lightbikes {
class MoveLightbikesSystem : public yage::GameSystem {
    void fixedUpdate(yage::World &world) override;
};
}

#endif //LIGHTBIKES_SYSTEMS_HPP
