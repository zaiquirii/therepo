#include <lightbikes/components/Position.hpp>
#include <lightbikes/input/InputState.hpp>
#include <lightbikes/rendering/Renderable.hpp>
#include <lightbikes/AppConfig.hpp>
#include <lightbikes/rendering/Camera.hpp>
#include <lightbikes/components/Lightbike.hpp>
#include <lightbikes/components/LightbikePath.hpp>
#include "LightbikeDuelState.hpp"
#include <SDL.h>

namespace lightbikes {
yage::World::entity_type createLightbike(yage::World &world, float x, float y, float speed, Lightbike::Direction dir) {
    const auto lightbike = world.create();
    world.emplace<Position>(lightbike, x, y);
    world.emplace<Renderable>(lightbike, Renderable::Type::Lightbike);
    world.emplace<Lightbike>(lightbike, speed, dir);
    LightbikePath path = {
            .lastDirection = Lightbike::Direction::None,
            .color = {0x00, 0xFF, 0x00}
    };
    world.emplace<LightbikePath>(lightbike, std::move(path));
    return lightbike;
}

void createCamera(yage::World &world, float width, float height) {
    const auto camera = world.create();
    world.emplace<Position>(camera, width / 2, height / 2);
    world.emplace<Camera>(camera, width, height);
}

void LightbikeDuelState::setup(yage::World &world) {
    auto &config = world.ctx<AppConfig>().lightbike;
    createCamera(world, config.worldDim.x, config.worldDim.y);

    auto lightbike = createLightbike(world, 0, 0, config.bikeSpeed, Lightbike::Direction::Right);
    lightbikes_.push_back(lightbike);

    lightbike = createLightbike(world, config.worldDim.x / 2, config.worldDim.y / 2, config.bikeSpeed,
                                Lightbike::Direction::Left);
    lightbikes_.push_back(lightbike);
}

bool LightbikeDuelState::fixedUpdate(yage::World &world) {
    // Get Player Input
    // Apply Input to lightbikes
    auto &input = world.ctx<InputState>();
    for (int i = 0; i < lightbikes_.size(); i++) {
        auto playerInput = input.getPlayerInput(i);
        auto &lightbike = world.get<Lightbike>(lightbikes_[i]);

        bool isPlayerInputHorizontal = playerInput.direction == Lightbike::Left ||
                                       playerInput.direction == Lightbike::Right;
        bool isLightbikeDirectionHorizontal = lightbike.direction == Lightbike::Left ||
                                              lightbike.direction == Lightbike::Right;
        // Don't change bike direction if input is on same axis
        // Basically, only 90 degree turns
        if (isPlayerInputHorizontal != isLightbikeDirectionHorizontal) {
            world.get<Lightbike>(lightbikes_[i]).direction = playerInput.direction;
        }
    }
    return !input.quitRequested;
}
}
