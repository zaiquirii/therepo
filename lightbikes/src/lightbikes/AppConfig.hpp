#ifndef LIGHTBIKES_APPCONFIG_HPP
#define LIGHTBIKES_APPCONFIG_HPP

#include <yage/yage.hpp>
#include <yaml-cpp/yaml.h>

namespace lightbikes {
struct LightbikeConfig {
    yage::Vec2 worldDim;
    float bikeSpeed;
};

struct AppConfig {
    yage::window::WindowConfig window;
    LightbikeConfig lightbike;
};
}

namespace YAML {
template <>
struct convert<lightbikes::AppConfig> {
    static bool decode(const Node &node, lightbikes::AppConfig &rhs) {
        rhs.window = node["window"].as<yage::window::WindowConfig>();
        rhs.lightbike.worldDim = node["lightbikes"]["worldDim"].as<yage::Vec2>();
        rhs.lightbike.bikeSpeed = node["lightbikes"]["bikeSpeed"].as<float>();
        return true;
    }
};
}

#endif //LIGHTBIKES_APPCONFIG_HPP
