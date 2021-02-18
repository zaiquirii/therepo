//
// Created by Zachary Smith on 2/16/21.
//

#include "RenderSystem.hpp"
#include "Camera.hpp"
#include <lightbikes/AppConfig.hpp>
#include <lightbikes/components/Position.hpp>
#include <lightbikes/rendering/Renderable.hpp>

namespace lightbikes {
void drawRenderables(SDL_Renderer *renderer, yage::Mat4 &projection, yage::World &entities);

void RenderSystem::setup(yage::World &world) {
    auto &config = world.ctx<AppConfig>();
    window_ = yage::window::create_window(config.window);
    renderer_ = SDL_CreateRenderer(window_, -1, 0);
    windowDims_ = {(float) config.window.width, (float) config.window.height};
}

void RenderSystem::fixedUpdate(yage::World &world) {
    SDL_SetRenderDrawColor(renderer_, 0, 0, 0, 255);
    SDL_RenderClear(renderer_);

    // Find Camera
    // Currently assuming only one camera
    auto cameraEnt = world.view<const Position, const Camera>().front();
    auto cameraPos = world.get<Position>(cameraEnt);
    auto camera = world.get<Camera>(cameraEnt);
    auto scale = windowDims_.x / camera.width;
    SDL_RenderSetScale(renderer_, scale, scale);
    auto projectionMatrix =  yage::scale({scale, scale, 1});

    drawRenderables(renderer_, projectionMatrix, world);

    SDL_RenderPresent(renderer_);
}

void RenderSystem::tearDown(yage::World &world) {
    SDL_DestroyRenderer(renderer_);
    SDL_DestroyWindow(window_);
    // TODO: Move this outside of the renderer, also using sdl for input and other things
    // doesn't make sense to do that here
    SDL_Quit();
}

// Internal render functions

void drawRenderables(SDL_Renderer *renderer, yage::Mat4 &projection, yage::World &world) {
    auto view = world.view<const Position, const Renderable>();
    for (auto[ent, pos, renderable]: view.each()) {
        switch (renderable.type) {
            case Renderable::Lightbike:
                auto newPos = projection * pos.asVec();
                SDL_SetRenderDrawColor(renderer, 0xFF, 0xFF, 0xFF, 0xFF);
                SDL_FRect rect{newPos.x, newPos.y, 1, 1};
//                SDL_FRect rect{newPos.x * 1280, newPos.y * 960, 1, 1};
                SDL_RenderFillRectF(renderer, &rect);
                break;
        }
    }
}
}
