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
            currentSize_(0) { /* empty */ }

    void push(Type elem) {
        // Move head
        head_ = (head_ + 1) % maxSize_;
        internalBuffer_[head_] = elem;
        if (currentSize_ != maxSize_) {
            currentSize_++;
        }
    }

    /// Offset = how many elements back we should go
    Type get(int offset) const {
        assert(offset < currentSize_);
        size_t index = head_ < offset
                       ? head_ + maxSize_ - offset
                       : head_ - offset;
        return internalBuffer_[index];
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
};
}


#endif //LIGHTBIKES_RINGBUFFER_HPP
