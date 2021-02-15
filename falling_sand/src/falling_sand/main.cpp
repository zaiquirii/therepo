#include <yage/yage.hpp>
#include <yaml-cpp/yaml.h>
#include <falling_sand/SandboxConfig.hpp>
#include <falling_sand/sim/CellSim.hpp>
#include <falling_sand/ui/Toolbox.hpp>
#include "MainState.hpp"
#include "CellSystem.hpp"
#include "RenderSystem.hpp"

using namespace falling_sand;

int main(int argc, char *args[]) {
    auto config = YAML::LoadFile("../assets/config/falling_sand_config.yml").as<FallingSandConfig>();
    auto sim = CellSim(config.sandbox.width, config.sandbox.height);
    Toolbox toolbox;

    yage::Game game = yage::Game();
    game.world().resources().set(&config);
    game.world().resources().set(&sim);
    game.world().resources().set(&toolbox);
    game.addSystem(new CellSystem());
    game.addSystem(new RenderSystem());
    game.setInitialState(new MainState());
    game.run();

    return 0;
}