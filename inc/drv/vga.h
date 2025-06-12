#ifndef VGA_H
#define VGA_H
#include <types.h>

#define VGA_CRTC_ADDR 0x3D4
#define VGA_CRTC_DATA 0x3D5

typedef enum vga_color {
    black, blue, green, cyan, red, magenta, brown, light_gray, dark_gray, light_blue, light_green, light_cyan, light_red, light_magenta, yellow, white
} vga_color;

char* vga_get_color_ptr(int x, int y);
char* vga_get_char_ptr(int x, int y);
u8 vga_get_color_attr(vga_color fg, vga_color bg);
void vga_scroll_up(vga_color fg, vga_color bg);
void vga_next_line(vga_color fg, vga_color bg);
void vga_next_char(vga_color fg, vga_color bg);
void vga_write_char(vga_color fg, vga_color bg, char c);
void vga_cursor_set_pos(int x, int y);
void vga_write_char_line(vga_color fg, vga_color bg, char c);
void vga_write_str(vga_color fg, vga_color bg, const char* s);
void vga_write_str_line(vga_color fg, vga_color bg, const char* s);
void vga_write_empty_line(vga_color fg, vga_color bg);
void vga_clear_line(vga_color fg, vga_color bg, int y);
void vga_clear(vga_color fg, vga_color bg);
#endif