#include "systems.hpp"
#include <iostream>
#include <lightbikes/components/Position.hpp>
#include <lightbikes/components/Lightbike.hpp>
#include <lightbikes/components/LightbikePath.hpp>

namespace lightbikes {
void updatePosition(float dt, Position &pos, Lightbike &bike);

void updatePath(Position &, Lightbike &, LightbikePath &);

void checkCollisions(yage::World &, Position &, Lightbike &);

void LightbikeSystem::fixedUpdate(yage::World &world) {
    auto &time = world.ctx<yage::Time>();
    auto view = world.view<Position, Lightbike, LightbikePath>();
    for (auto[ent, pos, bike, path]: view.each()) {
        updatePosition(time.fixedDelta(), pos, bike);
        updatePath(pos, bike, path);
        checkCollisions(world, pos, bike);
    }
}

void updatePosition(float dt, Position &pos, Lightbike &bike) {
    float xSpeed, ySpeed = 0;
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

    pos.x += bike.speed * xSpeed * dt;
    pos.y += bike.speed * ySpeed * dt;
}

void updatePath(Position &pos, Lightbike &bike, LightbikePath &path) {
    yage::Vec2 newPos = {pos.x, pos.y};
    if (path.lastDirection == bike.direction && path.points.size() > 1) {
        path.points.back() = newPos;
    } else {
        path.lastDirection = bike.direction;
        if (path.points.size() == 0 || path.points.back() != newPos) {
            path.points.push_back(newPos);
        }
    }
}

void checkCollisions(yage::World &world, Position &pos, Lightbike &bike) {
    float bikeSize = 1;
    float pathSize = 1;
    // Negating here as we currently have pos y going down
    yage::BoundingBox bikeBox = {pos.x, -pos.y, pos.x + bikeSize, -(pos.y + bikeSize)};
    for (auto[ent, path]: world.view<LightbikePath>().each()) {
        if (path.points.size() <= 3) {
            continue;
        }
        yage::BoundingBox segmentBox;
        for (int i = 0; i < path.points.size() - 3; i++) {
            auto start = path.points[i];
            auto end = path.points[i + 1];
            if (start.x == end.x) {
                if (end.y < start.y) {
                    segmentBox = {end.x, -end.y, start.x + pathSize, -(start.y + pathSize)};
                } else {
                    segmentBox = {start.x, -start.y, end.x + pathSize, -(end.y + pathSize)};
                }
            } else {
                if (end.x < start.x) {
                    segmentBox = {end.x, -end.y, start.x, -start.y};
                } else {
                    segmentBox = {start.x, -start.y, end.x, -end.y};
                }
            }

            // TODO: This would be a great place for an event system,
            // Not what I am focusing on right now though
            if (bikeBox.intersects(segmentBox)) {
                bike.dead = true;
                std::cout << "Collision" << std::endl;
            }
        }
    }
}
}
