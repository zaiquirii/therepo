#ifndef LIGHTBIKES_LIGHTBIKEDUELSTATE_HPP
#define LIGHTBIKES_LIGHTBIKEDUELSTATE_HPP


#include <yage/yage.hpp>

namespace lightbikes {
class LightbikeDuelState : public yage::GameState {
    void setup(yage::World &world) override;
    bool fixedUpdate(yage::World &world) override;
private:
//    std::vector<
};
}


#endif //LIGHTBIKES_LIGHTBIKEDUELSTATE_HPP
