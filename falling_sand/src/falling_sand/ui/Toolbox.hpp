#ifndef SRC_FALLING_SAND_TOOLBOX_HPP
#define SRC_FALLING_SAND_TOOLBOX_HPP


#include <yage/NoCopy.hpp>
#include "Brush.hpp"

namespace falling_sand {
class Toolbox : yage::NoCopy {
public:
    Toolbox();

    bool takeInput(InputSystem &input);

    void render(SDL_Renderer *renderer);

    Brush &currentBrush();

private:
    void renderCell(SDL_Renderer *renderer, int x, int y, Cell &cell, int border);

    std::vector<Cell> cells_;
    Point location_;
    int selectedCell_;
    int highlightedCell_;
    bool mouseWasDown_;
    SDL_Rect rect_;
    Brush currentBrush_;
};
}


#endif //SRC_FALLING_SAND_TOOLBOX_HPP
