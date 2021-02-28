#ifndef LIGHTBIKES_CONNECTIONMANAGER_HPP
#define LIGHTBIKES_CONNECTIONMANAGER_HPP

#include <string>
#include <vector>
#include <enet/enet.h>
#include "PlayerInputPacket.hpp"

namespace lightbikes {
enum ConnectionType {
    Host, Client
};

struct ConnectionArgs {
    ConnectionType connectionType;
    unsigned short port;
    std::string host;
};

/**
 * The ConnectionManager handles connections between clients
 */
class ConnectionManager {
public:
    void initConnection(ConnectionArgs &args);
    ~ConnectionManager();
//    void syncInput(PlayerInputPacket packet);
//    void closeConnection();
private:
    ENetEvent enetEvent_;
    ENetHost *enetHost_ = nullptr;
    ENetPeer *enetPeer_ = nullptr;
    std::vector<PlayerInputPacket> inputBuffer_ = std::vector<PlayerInputPacket>(10);
};
}

#endif //LIGHTBIKES_CONNECTIONMANAGER_HPP
