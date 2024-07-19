#pragma once
#include <types.h>

#define VGA_CRTC_ADDR 0x3D4
#define VGA_CRTC_DATA 0x3D5

typedef struct vga_size
{
    int w;
    int h;
} vga_size;

typedef struct vga_pos
{
    int x;
    int y;
} vga_pos;

typedef struct vga_info
{
    char* vmem;
    vga_size size;
    vga_pos pos;
} vga_info;

typedef struct vga_color
{
    enum { black, blue, green, cyan, red, magenta, brown, light_gray, dark_gray, light_blue, light_green, light_cyan, light_red, light_magenta, yellow, white } fg, bg;
} vga_color;

void vga_initialize(vga_info* info, char* vmem, vga_pos pos, vga_size size);
char* vga_get_color_ptr(vga_info* info, vga_pos pos);
char* vga_get_char_ptr(vga_info* info, vga_pos pos);
u8 vga_get_color_attr(vga_color color);
void vga_scroll_up(vga_info* info, vga_color color);
void vga_next_line(vga_info* info, vga_color color);
void vga_next_char(vga_info* info, vga_color color);
void vga_write_char(vga_info* info, vga_color color, char c);
void vga_cursor_set_pos(vga_info* info, vga_pos pos);
void vga_write_char_line(vga_info* info, vga_color color, char c);
void vga_write_str(vga_info* info, vga_color color, const char* s);
void vga_write_str_line(vga_info* info, vga_color color, const char* s);
void vga_write_empty_line(vga_info* info, vga_color color);
void vga_clear_line(vga_info* info, vga_color color, int y);
void vga_clear(vga_info* info, vga_color color);