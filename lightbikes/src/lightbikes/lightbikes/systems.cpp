#include "systems.hpp"
#include <lightbikes/components/Position.hpp>
#include <lightbikes/components/Lightbike.hpp>

namespace lightbikes {
void MoveLightbikesSystem::fixedUpdate(yage::World &world) {
    auto &time = world.ctx<yage::Time>();
    auto view = world.view<Position, Lightbike>();
    for (auto [ent, pos, bike]: view.each()) {
        float xSpeed, ySpeed;
        switch (bike.direction) {
            case Lightbike::Left:
                xSpeed = -1;
                ySpeed = 0;
                break;
            case Lightbike::Right:
                xSpeed = 1;
                ySpeed = 0;
                break;
            case Lightbike::Up:
                xSpeed = 0;
                ySpeed = -1;
                break;
            case Lightbike::Down:
                xSpeed = 0;
                ySpeed = 1;
                break;
        }

        pos.x += bike.speed * xSpeed * time.fixedDelta();
        pos.y += bike.speed * ySpeed * time.fixedDelta();
    }
}
}
