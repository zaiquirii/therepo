#include <SDL.h>
#include <yage/yage.hpp>
#include <yaml-cpp/yaml.h>
#include <falling_sand/sim/cell.hpp>
#include <falling_sand/ui/InputSystem.hpp>
#include <falling_sand/sim/SandboxConfig.hpp>
#include <falling_sand/sim/CellSim.hpp>
#include <falling_sand/ui/Toolbox.hpp>
#include "MainState.hpp"
#include "CellSystem.hpp"
#include "RenderSystem.hpp"

using namespace falling_sand;

int main(int argc, char *args[]) {
    auto config = YAML::LoadFile("../assets/config/sandbox_config.yml").as<SandboxConfig>();
    auto sim = CellSim(config.width, config.height);
    Toolbox globalToolbox;

    yage::Game game = yage::Game();
    game.world().resources().set(&config);
    game.world().resources().set(&sim);
    game.world().resources().set(&globalToolbox);
    game.addSystem(new CellSystem());
    game.addSystem(new RenderSystem());
    game.setInitialState(new MainState(
            {1290, 960},
            {sim.width, sim.height},
            sim));
    game.run();

    return 0;
}