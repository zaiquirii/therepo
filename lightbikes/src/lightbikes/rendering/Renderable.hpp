#ifndef LIGHTBIKES_RENDERABLE_HPP
#define LIGHTBIKES_RENDERABLE_HPP

namespace lightbikes {
/*
 * Represents something that can be rendered, this is intended to be replaced
 * by something like sprite or a mesh later on, but for now its going here.
 */
struct Renderable {
    enum Type {
        Lightbike
    };

    Type type;
};
}

#endif //LIGHTBIKES_RENDERABLE_HPP
