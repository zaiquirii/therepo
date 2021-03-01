#include <gtest/gtest.h>
#include "RingBuffer.hpp"

using namespace lightbikes;

TEST(RingBuffer, constructor) {
    RingBuffer<int> buffer(10);

    ASSERT_EQ(buffer.size(), 0);
}

TEST(RingBuffer, pushPopWorks) {
    RingBuffer<int> buffer(10);

    for (int i = 0; i < 10; i++) {
        buffer.push(i);
        ASSERT_EQ(buffer.size(), i + 1);
        ASSERT_EQ(buffer.get(0), i);
    }
    ASSERT_EQ(buffer.size(), 10);

    for (int i = 0; i < 10; i++) {
        buffer.push(i + 10);
        ASSERT_EQ(buffer.size(), 10);
        ASSERT_EQ(buffer.get(0), i + 10);
        ASSERT_EQ(buffer.get(9), i + 1);
    }
}
