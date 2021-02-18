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

    yage::Game game = yage::Game();
    game.world().set<FallingSandConfig>(config);
    game.world().set<CellSim>(config.sandbox.width, config.sandbox.height);
    game.world().set<Toolbox>();
    game.addSystem(new CellSystem());
    game.addSystem(new RenderSystem());
    game.setInitialState(new MainState());
    game.run();

    return 0;
}