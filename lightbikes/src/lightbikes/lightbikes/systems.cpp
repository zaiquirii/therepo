#include "systems.hpp"
#include <iostream>
#include <lightbikes/components/Position.hpp>
#include <lightbikes/components/Lightbike.hpp>
#include <lightbikes/components/LightbikePath.hpp>
#include <lightbikes/AppConfig.hpp>

namespace lightbikes {
/// Return true if bike teleported
bool updatePosition(float dt, yage::Vec2 worldDims, Position &pos, Lightbike &bike) {
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

    bool teleported = false;
    pos.x += bike.speed * xSpeed * dt;
    if (pos.x < 0) {
        pos.x = worldDims.width;
        teleported = true;
    } else if (pos.x > worldDims.width) {
        pos.x = 0;
        teleported = true;
    }

    pos.y += bike.speed * ySpeed * dt;
    // TODO: Figure out why a line on the top can be passed through
    if (pos.y < 0) {
        pos.y = worldDims.height;
        teleported = true;
    } else if (pos.y > worldDims.height) {
        pos.y = 0;
        teleported = true;
    }
    return teleported;
}

void updatePath(Position &pos, Lightbike &bike, LightbikePath &path, bool teleported) {
    yage::Vec2 newPos = {pos.x, pos.y};
    if (path.lastDirection == bike.direction && path.points.size() > 1 && !teleported) {
        path.points.back() = newPos;
    } else {
        path.lastDirection = bike.direction;
        if (path.points.size() == 0 || path.points.back() != newPos) {
            path.points.push_back(newPos);
            path.points.push_back(newPos);
        }
    }
}

void checkCollisions(yage::World &world, yage::World::entity_type &ent, Position &pos, Lightbike &bike) {
    float bikeSize = 1;
    float pathSize = 1;
    // Negating here as we currently have pos y going down
    yage::BoundingBox bikeBox = {pos.x, -pos.y, pos.x + bikeSize, -(pos.y + bikeSize)};
    for (auto[pathEnt, path]: world.view<LightbikePath>().each()) {
        // skip the last segment if its tied to the same ent. Makes collision checks a bit easier
        int pointsToSkip = (ent == pathEnt) ? 4 : 1;
        if (path.points.size() <= pointsToSkip) {
            continue;
        }
        yage::BoundingBox segmentBox;
        for (int i = 0; i < path.points.size() - pointsToSkip; i+=2) {
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
                    segmentBox = {end.x, -end.y, start.x + pathSize, -(start.y + pathSize)};
                } else {
                    segmentBox = {start.x, -start.y, end.x + pathSize, -(end.y + pathSize)};
                }
            }

            // TODO: This would be a great place for an event system,
            // Not what I am focusing on right now though
            if (bikeBox.intersects(segmentBox)) {
                bike.isDead = true;
//                std::cout << "Collision" << std::endl;
            }
        }
    }
}

void LightbikeSystem::fixedUpdate(yage::World &world) {
    auto &time = world.ctx<yage::Time>();
    auto view = world.view<Position, Lightbike, LightbikePath>();
    auto dims = world.ctx<AppConfig>().lightbike.worldDim;
    for (auto[ent, pos, bike, path]: view.each()) {
        bool teleported = updatePosition(time.fixedDelta(), dims, pos, bike);
        updatePath(pos, bike, path, teleported);
        checkCollisions(world, ent, pos, bike);
    }
}
}
