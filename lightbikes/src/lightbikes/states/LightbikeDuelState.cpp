#include <lightbikes/components/Position.hpp>
#include <lightbikes/input/InputState.hpp>
#include <lightbikes/rendering/Renderable.hpp>
#include <lightbikes/AppConfig.hpp>
#include <lightbikes/rendering/Camera.hpp>
#include <lightbikes/components/Lightbike.hpp>
#include "LightbikeDuelState.hpp"

namespace lightbikes {
void createLightbike(yage::World &world, float x, float y, float speed, Lightbike::Direction dir) {
    const auto lightbike = world.create();
    world.emplace<Position>(lightbike, x, y);
    world.emplace<Renderable>(lightbike, Renderable::Type::Lightbike);
    world.emplace<Lightbike>(lightbike, speed, dir);
}

void createCamera(yage::World &world, float width, float height) {
    const auto camera = world.create();
    world.emplace<Position>(camera, width / 2, height / 2);
    world.emplace<Camera>(camera, width, height);
}

void LightbikeDuelState::setup(yage::World &world) {
//    auto &config = world.resources().get<AppConfig>().lightbike;
    auto &config = world.ctx<AppConfig>().lightbike;
    createCamera(world, config.worldDim.x, config.worldDim.y);
    createLightbike(world, 0, 0, config.bikeSpeed, Lightbike::Direction::Down);
}

bool LightbikeDuelState::fixedUpdate(yage::World &world) {
    auto &time = world.ctx<yage::Time>();
    auto &input = world.ctx<InputState>();



    // Get Player Input
    // Apply Input to lightbikes

    return !input.quitRequested;
}
}
