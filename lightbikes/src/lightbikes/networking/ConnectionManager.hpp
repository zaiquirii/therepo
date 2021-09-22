#ifndef LIGHTBIKES_CONNECTIONMANAGER_HPP
#define LIGHTBIKES_CONNECTIONMANAGER_HPP

#include <string>
#include <vector>
#include <enet/enet.h>
#include "GameStatePacket.hpp"

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
//    void syncInput(GameStatePacket packet);
//    void closeConnection();
private:
    ENetEvent enetEvent_;
    ENetHost *enetHost_ = nullptr;
    ENetPeer *enetPeer_ = nullptr;
    std::vector<GameStatePacket> inputBuffer_ = std::vector<GameStatePacket>(10);
};
}

#endif //LIGHTBIKES_CONNECTIONMANAGER_HPP
