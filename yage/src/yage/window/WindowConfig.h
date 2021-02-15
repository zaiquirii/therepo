#ifndef YAGE_WINDOWCONFIG_H
#define YAGE_WINDOWCONFIG_H

#include <yaml-cpp/yaml.h>

namespace yage::window {
    struct WindowConfig {
        std::string title;
        int width;
        int height;
    };
}

namespace YAML {
    using namespace yage::window;

    template<>
    struct convert<WindowConfig> {
        static bool decode(const Node& node, WindowConfig& rhs) {
            rhs.title = node["title"].as<std::string>();
            rhs.width = node["width"].as<int>();
            rhs.height = node["height"].as<int>();
            return true;
        }
    };
}

#endif //YAGE_WINDOWCONFIG_H
