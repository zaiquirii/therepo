#ifndef LIGHTBIKES_RENDERSYSTEM_HPP
#define LIGHTBIKES_RENDERSYSTEM_HPP


#include <SDL.h>
#include <yage/yage.hpp>

namespace lightbikes {
/*
 * Originally, I was going to write some kind of plugin system for rendering, but that is
 * currently complete overkill as I imagine everything I'm writing will include the same
 * types of things to render. So for now. The Render system will check/render all the
 * component types itself.
 */
class RenderSystem : public yage::GameSystem {
public:
    void setup(yage::World &world) override;

    void fixedUpdate(yage::World &world) override;

    void tearDown(yage::World &world) override;

private:
    SDL_Window *window_;
    SDL_Renderer *renderer_;
    yage::Vec2 windowDims_;
};
}

#endif //LIGHTBIKES_RENDERSYSTEM_HPP
