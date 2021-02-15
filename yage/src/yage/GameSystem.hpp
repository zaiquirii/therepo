#ifndef SRC_FALLING_SAND_GAMESYSTEM_HPP
#define SRC_FALLING_SAND_GAMESYSTEM_HPP

#include "World.hpp"

namespace yage {
class GameSystem {
public:
    virtual void setup(yage::World &world) {};
    virtual void update(yage::World &world) = 0;
    virtual void tearDown(yage::World &world) {};
    virtual ~GameSystem() {};
};
}

#endif //SRC_FALLING_SAND_GAMESYSTEM_HPP
