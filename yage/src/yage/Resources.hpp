#ifndef SRC_FALLING_SAND_RESOURCES_HPP
#define SRC_FALLING_SAND_RESOURCES_HPP

#include "Family.hpp"
#include "YageException.hpp"
#include <unordered_map>
#include <vector>
#include <exception>

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
    template<typename ResourceType>
    ResourceType &get() {
        static const TypeId resourceId = Family<Resources>::get<ResourceType>();
        auto value = cache_[resourceId];
        if (value == nullptr) {
            throw YageException("No resource has been set for value");
        }
        return *static_cast<ResourceType *>(value);
    }

    template<typename ResourceType>
    void set(ResourceType *resource) {
        static const TypeId resourceId = Family<Resources>::get<ResourceType>();
        if (cache_.size() <= resourceId) {
            cache_.resize(cache_.size() * 2, nullptr);
        }
        cache_.at(resourceId) = resource;
    }

private:
    // TODO: Make this configurable so there aren't resizes in the middle of the operations
    std::vector<void *> cache_ = std::vector<void *>(10);
};
}

#endif //SRC_FALLING_SAND_RESOURCES_HPP
