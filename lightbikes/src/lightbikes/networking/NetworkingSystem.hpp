#ifndef LIGHTBIKES_NETWORKINGSYSTEM_HPP
#define LIGHTBIKES_NETWORKINGSYSTEM_HPP


#include <yage/yage.hpp>

namespace lightbikes {
class NetworkingSystem : public yage::GameSystem {
public:
    NetworkingSystem(bool isHost);
    void setup(yage::World &world) override;
    void fixedUpdate(yage::World &world) override;
private:
    bool isHost_;
};
}


#endif //LIGHTBIKES_NETWORKINGSYSTEM_HPP
