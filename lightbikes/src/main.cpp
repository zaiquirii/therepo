#include <yage/yage.hpp>
#include <lightbikes/AppConfig.hpp>
#include <lightbikes/rendering/RenderSystem.hpp>
#include <lightbikes/states/LightbikeDuelState.hpp>
#include <lightbikes/input/InputSystem.hpp>
#include <lightbikes/lightbikes/systems.hpp>
#include <lightbikes/networking/NetworkingSystem.hpp>
#include <iostream>

using namespace lightbikes;

int main(int argc, char *args[]) {
    auto config = YAML::LoadFile("../assets/config/app_config.yml").as<AppConfig>();
    std::cout << "argsdfc: " << argc << std::endl;
    yage::Game game = yage::Game();
    game.world().set<AppConfig>(config);
    game.addSystem(new InputSystem());
    game.addSystem(new NetworkingSystem(argc == 1));
    game.addSystem(new LightbikeSystem());
    game.addSystem(new RenderSystem());
    game.setInitialState(new LightbikeDuelState());
    game.run();
    return 0;
}
