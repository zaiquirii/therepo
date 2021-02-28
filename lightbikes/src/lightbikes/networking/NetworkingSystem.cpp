#include "NetworkingSystem.hpp"
#include "ConnectionManager.hpp"

namespace lightbikes {
NetworkingSystem::NetworkingSystem(bool isHost) : isHost_(isHost) {}

void NetworkingSystem::setup(yage::World &world) {
    auto &connectionManager = world.set<ConnectionManager>();
    ConnectionArgs args = {
            .connectionType = isHost_ ? Host : Client,
            .port = 9000
    };
    connectionManager.initConnection(args);
}

void NetworkingSystem::fixedUpdate(yage::World &world) {
}
}
