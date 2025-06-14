#ifndef FB_H
#define FB_H
#include <types.h>

struct framebuffer;
typedef struct framebuffer framebuffer;

struct framebuffer {
    void* addr;
    u32 pitch;
    u32 width;
    u32 height;
    u8 bits_per_pixel;
    u8 red_field_position;
    u8 red_mask_size;
    u8 green_field_position;
    u8 green_mask_size;
    u8 blue_field_position;
    u8 blue_mask_size;
    void (*put_pixel)(framebuffer*, u32, u32, u32);
};

static inline u8 framebuffer_convert_color(u8 value, u32 n) {
    if (n == 8) return value;
    return (value * ((1U << n) - 1) + 128) >> 8;
}

void framebuffer_put_pixel32(framebuffer* fb, u32 color, u32 x, u32 y);
void framebuffer_put_pixel24(framebuffer* fb, u32 color, u32 x, u32 y);
void framebuffer_put_pixel16(framebuffer* fb, u32 color, u32 x, u32 y);
#endif