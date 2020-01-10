struct Buffer {
    uint64_t   len; // u64 doesnt work?!
    uint32_t    data;
};

void start();
uint64_t queue_read(char* filename);
// void free_buf(struct buffer *buf);
struct Buffer poll();
