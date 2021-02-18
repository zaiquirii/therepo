#ifndef SRC_FALLING_SAND_WORLD_HPP
#define SRC_FALLING_SAND_WORLD_HPP


/*
 * Holds all entities and resources
 */
#include "Resources.hpp"
#include <entt/entity/registry.hpp>

namespace yage {
using World = entt::registry;
//class World {
//public:
//    using Entities = entt::registry;
//
//    Resources &resources() { return resources_; }
//    Entities &entities() { return entities_; }
//
//private:
//    Resources resources_;
//    Entities entities_;
//};
}


#endif //SRC_FALLING_SAND_WORLD_HPP
