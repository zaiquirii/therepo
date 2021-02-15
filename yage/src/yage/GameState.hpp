#ifndef SRC_FALLING_SAND_GAMESTATE_HPP
#define SRC_FALLING_SAND_GAMESTATE_HPP

#include <SDL.h>
#include "World.hpp"

namespace yage {
class GameState {
public:
    virtual void setup(yage::World &world) {};

    virtual bool update(yage::World &world) = 0;

    virtual ~GameState() {};
};
}
#endif //SRC_FALLING_SAND_GAMESTATE_HPP
