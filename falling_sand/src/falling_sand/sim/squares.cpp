#include <falling_sand/sim/squares.hpp>

unsigned int getSquareColor(Square square) {
    switch (square) {
        case AIR:
            return 0xFF000000;
        case SAND:
            return 0xFFFFFFFF;
    }
}