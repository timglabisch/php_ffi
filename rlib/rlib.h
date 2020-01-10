struct Buffer {
    uint64_t   data;
    uint32_t   len; // u64 doesnt work?!
};

struct Buffer poll();
