#ifndef SRC_FALLING_SAND_RENDERSYSTEM_HPP
#define SRC_FALLING_SAND_RENDERSYSTEM_HPP


#include <SDL.h>
#include <yage/GameSystem.hpp>

namespace falling_sand {
class RenderSystem : public yage::GameSystem {
public:
    void setup(yage::World &world) override;
    void update(yage::World &world) override;
    void tearDown(yage::World &world) override;
private:
    SDL_Window *window_;
    SDL_Renderer *renderer_;
    SDL_Texture *texture_;
    unsigned int *pixels_;
};
}


#endif //SRC_FALLING_SAND_RENDERSYSTEM_HPP
