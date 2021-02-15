#ifndef SRC_FALLING_SAND_FAMILY_HPP
#define SRC_FALLING_SAND_FAMILY_HPP

namespace yage {

using TypeId = int;

template<typename FamilyType>
class Family {
    static TypeId counter_s;

public:
    template<typename MemberType>
    static TypeId get() {
        static const TypeId STATIC_ID = counter_s++;
        return STATIC_ID;
    }

    static int size() {
        return counter_s;
    }
};

template<typename FamilyType> TypeId Family<FamilyType>::counter_s;
}

#endif //SRC_FALLING_SAND_FAMILY_HPP
