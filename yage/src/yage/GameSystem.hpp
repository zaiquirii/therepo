#ifndef SRC_FALLING_SAND_GAMESYSTEM_HPP
#define SRC_FALLING_SAND_GAMESYSTEM_HPP

#include "World.hpp"

namespace yage {
class GameSystem {
public:
    virtual void setup(yage::World &world) {};
    virtual void update(yage::World &world) {};
    virtual void fixedUpdate(yage::World &world) {};
    // TODO: Do we need both tearDown and destructors, probably not
    virtual ~GameSystem() {};
    virtual void tearDown(yage::World &world) {};
};
}

#endif //SRC_FALLING_SAND_GAMESYSTEM_HPP
