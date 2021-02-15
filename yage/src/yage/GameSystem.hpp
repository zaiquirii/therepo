#ifndef SRC_FALLING_SAND_GAMESYSTEM_HPP
#define SRC_FALLING_SAND_GAMESYSTEM_HPP

#include "World.hpp"

namespace yage {
class GameSystem {
public:
    virtual bool setup(World &world) = 0;
    virtual void update(World &world) = 0;
    virtual ~GameSystem() {};
};
}

#endif //SRC_FALLING_SAND_GAMESYSTEM_HPP
