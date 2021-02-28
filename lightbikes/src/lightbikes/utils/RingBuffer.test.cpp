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
    }
    ASSERT_EQ(buffer.size(), 10);

    int count = 0;
    while (buffer.size() > 0) {
        auto value = buffer.pop();
        ASSERT_EQ(value, count);
        count++;
    }
}

TEST(RingBuffer, cannotExceedMaxSize) {
    RingBuffer<int> buffer(10);
    for (int i = 0; i< 10; i++) {
        buffer.push(i);
    }

    ASSERT_DEBUG_DEATH(buffer.push(11), ".*");
}