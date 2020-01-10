struct Buffer {
    uint32_t   data;
    uint32_t   len;
};

void start();
uint64_t queue_read(char* filename);
// void free_buf(struct buffer *buf);
struct Buffer poll();
