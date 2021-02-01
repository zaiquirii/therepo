//
// Created by Zachary Smith on 2/1/21.
//

#ifndef SRC_FALLING_SAND_INPUTSYSTEM_HPP
#define SRC_FALLING_SAND_INPUTSYSTEM_HPP


#include <SDL.h>

namespace falling_sand {
struct Point { int x; int y; };

class InputSystem {
public:
    void pollInput();
    bool quitRequested() { return quitRequested_; }
    bool mouseDown() { return mouseDown_; }
    Point mousePos(int windowWidth, int windowHeight, int simWidth, int simHeight);

private:
    bool quitRequested_;
    SDL_Event event_;
    Point rawMousePos_;
    bool mouseDown_;
};
}


#endif //SRC_FALLING_SAND_INPUTSYSTEM_HPP
