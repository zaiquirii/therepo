#ifndef SRC_FALLING_SAND_WORLD_HPP
#define SRC_FALLING_SAND_WORLD_HPP


/*
 * Holds all entities and resources
 */
#include "Resources.hpp"

namespace yage {
class World {
public:
    Resources &resources() { return resources_; }

private:
    Resources resources_;
};
}


#endif //SRC_FALLING_SAND_WORLD_HPP
