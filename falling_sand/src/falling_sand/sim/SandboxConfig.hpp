#ifndef SRC_FALLING_SAND_SANDBOXCONFIG_HPP
#define SRC_FALLING_SAND_SANDBOXCONFIG_HPP

#include <yaml-cpp/yaml.h>

namespace falling_sand {
    struct SandboxConfig {
        int width;
        int height;
    };
}

namespace YAML {
    template<>
    struct convert<falling_sand::SandboxConfig> {
        static bool decode(const Node &node, falling_sand::SandboxConfig &rhs) {
            rhs.width = node["width"].as<int>();
            rhs.height = node["height"].as<int>();
            return true;
        }
    };
}

#endif //SRC_FALLING_SAND_SANDBOXCONFIG_HPP
