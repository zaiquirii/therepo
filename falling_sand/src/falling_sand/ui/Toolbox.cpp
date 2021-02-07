//
// Created by Zachary Smith on 2/4/21.
//

#include "Toolbox.hpp"

namespace falling_sand {
int CELL_SIZE = 25;
int CELL_SPACING = 15;

Toolbox::Toolbox() {
    cells_.push_back(EMPTY_CELL);
    cells_.push_back(SAND_CELL);
    cells_.push_back(WATER_CELL);
    cells_.push_back(WALL_CELL);
    cells_.push_back(OIL_CELL);
    selectedCell_ = 2;
    highlightedCell_ = 2;
    location_ = {CELL_SPACING, CELL_SPACING};
}


bool Toolbox::takeInput(InputSystem &input) {
    Point mousePos = input.mousePos();
    if (mousePos.x >= location_.x &&
        mousePos.x < location_.x + CELL_SIZE &&
        mousePos.y >= location_.y &&
        mousePos.y < location_.y + ((CELL_SIZE + CELL_SPACING) * cells_.size())) {
        int offset = mousePos.y - location_.y;
        int cellIndex = offset / (CELL_SIZE + CELL_SPACING);
        if (offset % (CELL_SIZE + CELL_SPACING) < CELL_SIZE) {
            highlightedCell_ = cellIndex;
        } else {
            highlightedCell_ = -1;
        }

        if (highlightedCell_ != -1 && input.mouseDown()) {
            selectedCell_ = highlightedCell_;
        }
        return highlightedCell_ != -1;
    }
    return false;
}

Brush &Toolbox::currentBrush() {
    currentBrush_.particle = cells_[selectedCell_];
    currentBrush_.size = 8;
    currentBrush_.type = selectedCell_ == 0 ? Fill : FillEmpty;
    return currentBrush_;
}

void Toolbox::render(SDL_Renderer *renderer) {
    for (int i = 0; i < cells_.size(); i++) {
        renderCell(renderer, location_.x,
                   location_.y + (CELL_SIZE + CELL_SPACING) * i,
                   cells_[i], selectedCell_ == i || highlightedCell_ == i ? 5 : 1);
    }
}

void Toolbox::renderCell(SDL_Renderer *renderer, int x, int y, Cell &cell, int border) {
    int color = getCellColor(cell);
    short red = (color & 0x00FF0000) >> 16;
    short green = (color & 0x0000FF00) >> 8;
    short blue = (color & 0x000000FF);

    rect_.x = x;
    rect_.y = y;
    rect_.w = CELL_SIZE;
    rect_.h = CELL_SIZE;
    SDL_SetRenderDrawColor(renderer, 255, 255, 255, 255);
    SDL_RenderFillRect(renderer, &rect_);

    rect_.x = x + border;
    rect_.y = y + border;
    rect_.w = CELL_SIZE - (border * 2);
    rect_.h = CELL_SIZE - (border * 2);
    SDL_SetRenderDrawColor(renderer, red, green, blue, 255);
    SDL_RenderFillRect(renderer, &rect_);
}

}
