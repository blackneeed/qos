#include <types.h>
#include <struct/fb.h>

void framebuffer_put_pixel32(framebuffer* fb, u32 color, u32 x, u32 y)
{
    if (x >= fb->width || y >= fb->height) return;
    *(u32*)((char*)fb->addr + (y * fb->pitch) + (x * 4)) = (framebuffer_convert_color((color >> 16) & 0xFF, fb->red_mask_size) << fb->red_field_position) | (framebuffer_convert_color((color >> 8) & 0xFF, fb->green_mask_size) << fb->green_field_position) | (framebuffer_convert_color(color & 0xFF, fb->blue_mask_size) << fb->blue_field_position);
}

void framebuffer_put_pixel24(framebuffer* fb, u32 color, u32 x, u32 y)
{
    if (x >= fb->width || y >= fb->height) return;
    u8 *pixel = (u8*)((char*)fb->addr + (y * fb->pitch) + (x * 3));

    pixel[0] = (framebuffer_convert_color((color) & 0xFF, fb->blue_mask_size) << fb->blue_field_position) >> fb->blue_field_position;
    pixel[1] = (framebuffer_convert_color((color >> 8) & 0xFF, fb->green_mask_size) << fb->green_field_position) >> fb->green_field_position;
    pixel[2] = (framebuffer_convert_color((color >> 16) & 0xFF, fb->red_mask_size) << fb->red_field_position) >> fb->red_field_position;
}

void framebuffer_put_pixel16(framebuffer* fb, u32 color, u32 x, u32 y)
{
    if (x >= fb->width || y >= fb->height) return;
    *(u16*)((char*)fb->addr + (y * fb->pitch) + (x * 2)) = (framebuffer_convert_color((color >> 16) & 0xFF, fb->red_mask_size) << fb->red_field_position) | (framebuffer_convert_color((color >> 8) & 0xFF, fb->green_mask_size) << fb->green_field_position) | (framebuffer_convert_color(color & 0xFF, fb->blue_mask_size) << fb->blue_field_position);
}