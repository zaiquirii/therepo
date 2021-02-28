#ifndef LIGHTBIKES_RINGBUFFER_HPP
#define LIGHTBIKES_RINGBUFFER_HPP

#include <vector>


namespace lightbikes {
template<typename Type>
class RingBuffer {
public:
    explicit RingBuffer(size_t size) :
            internalBuffer_(std::unique_ptr<Type[]>(new Type[size])),
            maxSize_(size),
            head_(0),
            tail_(0),
            currentSize_(0) { /* empty */ }

    void push(Type elem) {
        assert(currentSize_ != maxSize_);
        internalBuffer_[head_] = elem;
        head_ = (head_ + 1) % maxSize_;
        currentSize_++;
    }

    Type pop() {
        size_t oldTail = tail_;
        tail_ = (tail_ + 1) % maxSize_;
        currentSize_--;
        return std::move(internalBuffer_[oldTail]);
    }

    size_t size() const {
        return currentSize_;
    }

private:
    size_t maxSize_;
    size_t currentSize_;
    std::unique_ptr<Type[]> internalBuffer_;
    /// Index where things are added
    size_t head_;
    /// Index where things are removed
    size_t tail_;
};
}


#endif //LIGHTBIKES_RINGBUFFER_HPP
