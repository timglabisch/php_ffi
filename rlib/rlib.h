struct Buffer {
    uint8_t*   data;
    uint32_t   len; // u64 doesnt work?!
};

void start();
uint64_t queue_read(char* filename);
// void free_buf(struct buffer *buf);
struct Buffer poll();
