//
// Created by Zachary Smith on 2/16/21.
//

#include "RenderSystem.hpp"
#include "Camera.hpp"
#include <lightbikes/AppConfig.hpp>
#include <lightbikes/components/Position.hpp>
#include <lightbikes/rendering/Renderable.hpp>
#include <lightbikes/components/LightbikePath.hpp>

namespace lightbikes {
void drawRenderables(SDL_Renderer *renderer, yage::Mat4 &projection, yage::World &entities);

void drawPaths(SDL_Renderer *renderer, yage::Mat4 &projection, yage::World &entities);

void RenderSystem::setup(yage::World &world) {
    auto &config = world.ctx<AppConfig>();
    window_ = yage::window::create_window(config.window);
    renderer_ = SDL_CreateRenderer(window_, -1, 0);
    windowDims_ = {(float) config.window.width, (float) config.window.height};
}

void RenderSystem::update(yage::World &world) {
//    void RenderSystem::fixedUpdate(yage::World &world) {
    SDL_SetRenderDrawColor(renderer_, 0, 0, 0, 255);
    SDL_RenderClear(renderer_);

    // Find Camera
    // Currently assuming only one camera
    auto cameraEnt = world.view<const Position, const Camera>().front();
    auto cameraPos = world.get<Position>(cameraEnt);
    auto camera = world.get<Camera>(cameraEnt);
    auto scale = windowDims_.x / camera.width;
    SDL_RenderSetIntegerScale(renderer_, SDL_TRUE);
    SDL_RenderSetScale(renderer_, scale, scale);

    auto projectionMatrix = yage::translation({-cameraPos.x, -cameraPos.y, 0});

    drawPaths(renderer_, projectionMatrix, world);
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
                auto newPos = pos.asVec();
                SDL_SetRenderDrawColor(renderer, 0xFF, 0xFF, 0xFF, 0xFF);
                SDL_FRect rect{newPos.x, newPos.y, 1, 1};
                SDL_RenderFillRectF(renderer, &rect);
                break;
        }
    }
}

void drawPaths(SDL_Renderer *renderer, yage::Mat4 &projection, yage::World &world) {
    auto view = world.view<const LightbikePath>();
    SDL_FRect rect;
    float width = 1;
    for (auto [ent, path]: view.each()) {
        if (path.points.size() < 2) {
            continue;
        }
        for (int i = 0; i < path.points.size() - 1; i+=2) {
            auto start = path.points[i];
            auto end = path.points[i + 1];
            if (start.x == end.x) {
                if (end.y < start.y) {
                    rect.x = end.x;
                    rect.y = end.y;
                    rect.h = start.y - end.y + width;
                    rect.w = width;
                } else {
                    rect.x = start.x;
                    rect.y = start.y;
                    rect.h = end.y - start.y + width;
                    rect.w = width;
                }
            } else {
                if (end.x < start.x) {
                    rect.x = end.x;
                    rect.y = end.y;
                    rect.w = start.x - end.x + width;
                    rect.h = width;
                } else {
                    rect.x = start.x;
                    rect.y = start.y;
                    rect.w = end.x - start.x + width;
                    rect.h = width;
                }
            }
            auto color = path.color;
            SDL_SetRenderDrawColor(renderer, color[0], color[1], color[2], 0xFF);
            SDL_RenderFillRectF(renderer, &rect);
        }
    }
}
}
