#ifndef SRC_FALLING_SAND_RESOURCES_HPP
#define SRC_FALLING_SAND_RESOURCES_HPP

#include "Family.hpp"
#include <unordered_map>
#include <stdexcept>


/*
 * A resource is any object that should be made available to all systems/states.
 */
namespace yage {
class Resources {
public:
    /*
     * Make sure you are using auto & with this or just ResourceType&, otherwise you'll start getting copies
     * Not calling set(nullptr)
     */
    template <typename ResourceType>
    ResourceType &get() {
        static const TypeId resourceId = Family<Resources>::get<ResourceType>();
        auto value = cache_[resourceId];
        if (value == nullptr) {
            throw std::logic_error("No resource has been set for value");
        }
        return *static_cast<ResourceType *>(value);
    }

    template <typename ResourceType>
    void set(ResourceType *resource) {
        static const TypeId resourceId = Family<Resources>::get<ResourceType>();
        cache_[resourceId] = resource;
    }
private:
    std::unordered_map<TypeId, void*> cache_;
};
}

#endif //SRC_FALLING_SAND_RESOURCES_HPP
