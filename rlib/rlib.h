struct buffer {
    uint8_t*   data;
    uint64_t   len;
};

void start();
uint64_t queue_read(char* filename);
void free_buf(struct buffer *buf);
struct buffer poll();
