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
yage::World::entity_type
createLightbike(yage::World &world, float x, float y, float speed, Lightbike::Direction dir, Uint8 *color) {
    const auto lightbike = world.create();
    world.emplace<Position>(lightbike, x, y);
    world.emplace<Renderable>(lightbike, Renderable::Type::Lightbike);
    world.emplace<Lightbike>(lightbike, speed, dir);
    LightbikePath path = {
            .lastDirection = Lightbike::Direction::None,
            .color = {color[0], color[1], color[2]}
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
    resetDuel(world);
}

bool LightbikeDuelState::fixedUpdate(yage::World &world) {
    // Get Player Input
    // Apply Input to lightbikes
    auto &input = world.ctx<InputState>();
    bool shouldResetDuel = false;
    for (int i = 0; i < lightbikes_.size(); i++) {
        auto playerInput = input.getPlayerInput(i);
        if (playerInput.direction == Lightbike::None) {
            continue;
        }

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

        if (lightbike.isDead) {
            shouldResetDuel = true;
            break;
        }
    }
    if (shouldResetDuel) {
        resetDuel(world);

        // TODO: This shouldn't be necessary once everything (including timers) are setup
        for (int i = 0; i < lightbikes_.size(); i++) {
            PlayerInputState playerInput = input.getPlayerInput(i);
            playerInput.direction = Lightbike::None;
            input.setPlayerInput(i, playerInput);
        }
    }
    return !input.quitRequested;
}

void LightbikeDuelState::resetDuel(yage::World &world) {
    for (auto ent: lightbikes_) {
        world.destroy(ent);
    }
    lightbikes_.clear();

    auto &config = world.ctx<AppConfig>().lightbike;
    Uint8 color[]{0x00, 0xFF, 0xFF};
    auto lightbike = createLightbike(world, 0, 0,
                                     config.bikeSpeed, Lightbike::Right,
                                     color);
    lightbikes_.push_back(lightbike);

    Uint8 color2[]{0xFF, 0xFF, 0x00};
    lightbike = createLightbike(world, config.worldDim.x / 2, config.worldDim.y / 2,
                                config.bikeSpeed, Lightbike::Left,
                                color2);
    lightbikes_.push_back(lightbike);
}
}
