#include <types.h>
#include <struct/fb.h>

void framebuffer_put_pixel(framebuffer* fb, u32 color, u32 x, u32 y)
{
    if (x >= fb->width || y >= fb->height) return;
    *(u32*)((char*)fb->addr + (y * fb->pitch) + (x * fb->bytes_per_pixel)) = (framebuffer_convert_color((color >> 16) & 0xFF, fb->red_mask_size) << fb->red_field_position) | (framebuffer_convert_color((color >> 8) & 0xFF, fb->green_mask_size) << fb->green_field_position) | (framebuffer_convert_color(color & 0xFF, fb->blue_mask_size) << fb->blue_field_position);
}