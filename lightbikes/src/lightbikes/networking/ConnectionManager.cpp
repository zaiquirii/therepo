#include <yage/YageException.hpp>
#include "ConnectionManager.hpp"
#include <iostream>

namespace lightbikes {
ConnectionManager::~ConnectionManager() {
    if (enetPeer_) {
        enet_peer_disconnect(enetPeer_, 5000);
        enet_peer_reset(enetPeer_);
    }
    if (enetHost_) {
        enet_host_destroy(enetHost_);
    }

    enet_deinitialize();
}

void ConnectionManager::initConnection(lightbikes::ConnectionArgs &args) {
    if (enet_initialize() != 0) {
        throw yage::YageException("Enet failed to initialize");
    }

    ENetAddress address;
    if (args.connectionType == Host) {
//        enet_address_set_host(&address, "127.0.0.1");
        address.host = ENET_HOST_ANY;
        address.port = args.port;
        enetHost_ = enet_host_create(&address, 1, 1, 0, 0);
        if (enetHost_ == nullptr) {
            throw yage::YageException("Could not create host");
        }
        std::cout << "Starting as host" << std::endl;

        while (enetEvent_.type != ENET_EVENT_TYPE_CONNECT) {
            int value = enet_host_service(enetHost_, &enetEvent_, 1000);
            std::cout << "Value: " << value << std::endl;
        }
//        if (enet_host_service(enetHost_, &enetEvent_, 10000) <= 0 &&
//            enetEvent_.type != ENET_EVENT_TYPE_CONNECT) {
//        }
//
//        if (enet_host_service(enetHost_, &enetEvent_, 10000) <= 0 &&
//            enetEvent_.type != ENET_EVENT_TYPE_CONNECT) {
//            std::cout << "ENET EVENT " << enetEvent_.type << std::endl;
//            throw yage::YageException("Failed to establish connection in timee");
//        }
        std::cout << "Host Connection established" << std::endl;
        enetPeer_ = enetEvent_.peer;
    } else {
        enetHost_ = enet_host_create(nullptr, 1, 1, 0, 0);
        if (enetHost_ == nullptr) {
            throw yage::YageException("Could not create host");
        }
        enet_address_set_host(&address, "127.0.0.1");
        address.port = 9000;
        std::cout << "Starting as peer" << std::endl;
        enetPeer_ = enet_host_connect(enetHost_, &address, 1, 0);
        if (enetPeer_ == nullptr) {
            throw yage::YageException("Could not connect to host");
        }

        if (enet_host_service(enetHost_, &enetEvent_, 10000) <= 0 &&
            enetEvent_.type != ENET_EVENT_TYPE_CONNECT) {
            throw yage::YageException("Failed to establish connection in time");
        }
        std::cout << "Peer connection Established" << std::endl;
        enet_host_flush(enetHost_);
    }
}
}
