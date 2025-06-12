#include <types.h>
#include <drv/ioport.h>
#include <drv/vga.h>

static char* VGA_VMEM = (char*)0xB8000;
static int VGA_SIZE_W = 80;
static int VGA_SIZE_H = 25;
static int VGA_POS_X = 0;
static int VGA_POS_Y = 0;

char* vga_get_color_ptr(int x, int y)
{
    return VGA_VMEM + (x + (y * VGA_SIZE_W)) * 2 + 1;
}

char* vga_get_char_ptr(int x, int y)
{
    return VGA_VMEM + (x + (y * VGA_SIZE_W)) * 2;
}

u8 vga_get_color_attr(vga_color fg, vga_color bg)
{
    return fg | (bg << 4);
}

void vga_scroll_up(vga_color fg, vga_color bg)
{
    int srcx = 0;
    int srcy = 0;
    int dstx = 0;
    int dsty = 0;

    for (srcy = 1; srcy < VGA_SIZE_H; srcy++)
    {
        for (srcx = 0; srcx < VGA_SIZE_W; srcx++)
        {
            *vga_get_char_ptr(dstx, dsty) = *vga_get_char_ptr(srcx, srcy);
            *vga_get_color_ptr(dstx, dsty) = *vga_get_color_ptr(srcx, srcy);
        }
    }

    vga_clear_line(fg, bg, 0);
}

void vga_next_line(vga_color fg, vga_color bg)
{
    if (VGA_POS_Y + 1 >= VGA_SIZE_H)
    {
        vga_scroll_up(fg, bg);
    }
    else VGA_POS_Y++;
}

void vga_next_char(vga_color fg, vga_color bg)
{
    if (VGA_POS_X + 1 >= VGA_SIZE_W)
    {
        vga_next_line(fg, bg);
        VGA_POS_X = 0;
    }
    else VGA_POS_X++;
}

void vga_write_char(vga_color fg, vga_color bg, char c)
{
    switch (c)
    {
        case '\r':
            VGA_POS_X = 0;
            break;

        case '\n':
            vga_next_line(fg, bg);
            break;

        case '\b':
            if (VGA_POS_X > 0)
                VGA_POS_X--;
            else if (VGA_POS_Y > 0)
                VGA_POS_Y--;
            break;

        case '\t':
            for (int i = 0; i < 4; i++)
                vga_write_char(fg, bg, ' ');
            break;
        
        default:
            *vga_get_char_ptr(VGA_POS_X, VGA_POS_Y) = c;
            *vga_get_color_ptr(VGA_POS_X, VGA_POS_Y) = vga_get_color_attr(fg, bg);
            vga_next_char(fg, bg);
            break;
    }

    vga_cursor_set_pos(VGA_POS_X, VGA_POS_Y);
}

void vga_cursor_set_pos(int x, int y)
{
    u16 abs_pos = y * VGA_SIZE_W + x;
    io_outb(VGA_CRTC_ADDR, 0xE);
    io_outb(VGA_CRTC_DATA, (u8)(abs_pos >> 8));
    io_outb(VGA_CRTC_ADDR, 0xF);
    io_outb(VGA_CRTC_DATA, (u8)abs_pos);
}

void vga_write_char_line(vga_color fg, vga_color bg, char c)
{
    vga_write_char(fg, bg, c);
    vga_write_empty_line(fg, bg);
}

void vga_write_str(vga_color fg, vga_color bg, const char* s)
{
    for (int i = 0; s[i]; i++)
        vga_write_char(fg, bg, s[i]);
}

void vga_write_str_line(vga_color fg, vga_color bg, const char* s)
{
    vga_write_str(fg, bg, s);
    vga_write_empty_line(fg, bg);
}

void vga_write_empty_line(vga_color fg, vga_color bg)
{
    vga_write_str(fg, bg, "\r\n");    
}

void vga_clear_line(vga_color fg, vga_color bg, int y)
{
    int cx = 0;
    int cy = y;
    for (; cx < VGA_SIZE_W; cx++) {
        *vga_get_char_ptr(cx, cy) = ' ';
        *vga_get_color_ptr(cx, cy) = vga_get_color_attr(fg, bg);
    }
}

void vga_clear(vga_color fg, vga_color bg)
{
    for (int x = 0; x < VGA_SIZE_W; x++)
    {
        for (int y = 0; y < VGA_SIZE_H; y++)
        {
            *vga_get_char_ptr(x, y) = ' ';
            *vga_get_color_ptr(x, y) = vga_get_color_attr(fg, bg);
        }
    }
}