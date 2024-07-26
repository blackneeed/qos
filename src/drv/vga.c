#include <types.h>
#include <drv/ioport.h>
#include "vga.h"

void vga_initialize(vga_info* info, char* vmem, vga_pos pos, vga_size size)
{
    info->vmem = vmem;
    info->pos = pos;
    info->size = size;
}

char* vga_get_color_ptr(vga_info* info, vga_pos pos)
{
    return info->vmem + (pos.x + (pos.y * info->size.w)) * 2 + 1;
}

char* vga_get_char_ptr(vga_info* info, vga_pos pos)
{
    return info->vmem + (pos.x + (pos.y * info->size.w)) * 2;
}

u8 vga_get_color_attr(vga_color color)
{
    return color.fg | (color.bg << 4);
}

void vga_scroll_up(vga_info* info, vga_color color)
{
    vga_pos src = {};
    vga_pos dest = {};
    for (src.y = 1; src.y < info->size.h; src.y++)
    {
        for (src.x = 0; src.x < info->size.w; src.x++)
        {
            *vga_get_char_ptr(info, dest) = *vga_get_char_ptr(info, src);
            *vga_get_color_ptr(info, dest) = *vga_get_color_ptr(info, src);
        }
    }
    vga_clear_line(info, color, 0);
}

void vga_next_line(vga_info* info, vga_color color)
{
    if (info->pos.y + 1 >= info->size.h)
    {
        vga_scroll_up(info, color);
    }
    else info->pos.y++;
}

void vga_next_char(vga_info* info, vga_color color)
{
    if (info->pos.x + 1 >= info->size.w)
    {
        vga_next_line(info, color);
        info->pos.x = 0;
    }
    else info->pos.x++;
}

void vga_write_char(vga_info* info, vga_color color, char c)
{
    switch (c)
    {
        case '\r':
            info->pos.x = 0;
            break;

        case '\n':
            vga_next_line(info, color);
            break;

        case '\b':
            if (info->pos.x > 0)
                info->pos.x--;
            else if (info->pos.y > 0)
                info->pos.y--;
            break;

        case '\t':
            for (int i = 0; i < 4; i++)
                vga_write_char(info, color, ' ');
            break;
        
        default:
            *vga_get_char_ptr(info, info->pos) = c;
            *vga_get_color_ptr(info, info->pos) = vga_get_color_attr(color);
            vga_next_char(info, color);
            break;
    }

    vga_cursor_set_pos(info, info->pos);
}

void vga_cursor_set_pos(vga_info* info, vga_pos pos)
{
    u16 cursor_location = pos.y * info->size.w + pos.x;
    io_outb(VGA_CRTC_ADDR, 0xE);
    io_outb(VGA_CRTC_DATA, (u8)(cursor_location >> 8));
    io_outb(VGA_CRTC_ADDR, 0xF);
    io_outb(VGA_CRTC_DATA, (u8)cursor_location);
}

void vga_write_char_line(vga_info* info, vga_color color, char c)
{
    vga_write_char(info, color, c);
    vga_write_empty_line(info, color);
}

void vga_write_str(vga_info* info, vga_color color, const char* s)
{
    for (int i = 0; s[i]; i++)
        vga_write_char(info, color, s[i]);
}

void vga_write_str_line(vga_info* info, vga_color color, const char* s)
{
    vga_write_str(info, color, s);
    vga_write_empty_line(info, color);
}

void vga_write_empty_line(vga_info* info, vga_color color)
{
    vga_write_str(info, color, "\r\n");    
}

void vga_clear_line(vga_info* info, vga_color color, int y)
{
    vga_pos pos;
    pos.y = y;
    for (; pos.x < info->size.w; pos.x++) {
        *vga_get_char_ptr(info, pos) = ' ';
        *vga_get_color_ptr(info, pos) = vga_get_color_attr(color);
    }
}

void vga_clear(vga_info* info, vga_color color)
{
    vga_pos pos;
    for (pos.x = 0; pos.x < info->size.w; pos.x++)
    {
        for (pos.y = 0; pos.y < info->size.h; pos.y++)
        {
            *vga_get_char_ptr(info, pos) = ' ';
            *vga_get_color_ptr(info, pos) = vga_get_color_attr(color);
        }
    }
}