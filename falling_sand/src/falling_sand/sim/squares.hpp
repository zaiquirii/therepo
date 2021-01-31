//
// Created by Zachary Smith on 1/28/21.
//

#ifndef SRC_FALLING_SAND_SQUARES_HPP
#define SRC_FALLING_SAND_SQUARES_HPP

enum Square: short {
    AIR = 0,
    SAND = 1
};

unsigned int getSquareColor(Square square);

#endif //SRC_FALLING_SAND_SQUARES_HPP
