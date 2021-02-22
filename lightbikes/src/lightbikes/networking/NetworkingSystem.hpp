#ifndef LIGHTBIKES_NETWORKINGSYSTEM_HPP
#define LIGHTBIKES_NETWORKINGSYSTEM_HPP


#include <yage/yage.hpp>

namespace lightbikes {
class NetworkingSystem : public yage::GameSystem {
    void setup(yage::World &world) override;
    void fixedUpdate(yage::World &world) override;
    void tearDown(yage::World &world) override;
};
}


#endif //LIGHTBIKES_NETWORKINGSYSTEM_HPP
