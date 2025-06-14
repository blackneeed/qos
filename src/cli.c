#include <types.h>
#include <struct/font.h>
#include <struct/fb.h>

static struct {
    u32 x;
    u32 y;
    u32 cx;
    u32 cy;
    framebuffer* fb;
    font16* f16;
    u32 color;
    u32 last_color;
    u32 background_color;
    u32 last_background_color;
} CURRENT_CLI_CTX;

void cli_init(framebuffer* fb)
{
    CURRENT_CLI_CTX.f16 = NULL;
    CURRENT_CLI_CTX.fb = fb;
    CURRENT_CLI_CTX.x = 0;
    CURRENT_CLI_CTX.cx = 0;
    CURRENT_CLI_CTX.y = 0;
    CURRENT_CLI_CTX.cy = 0;
    CURRENT_CLI_CTX.color = 0;
    CURRENT_CLI_CTX.last_color = 0;
    CURRENT_CLI_CTX.background_color = 0;
    CURRENT_CLI_CTX.last_background_color = 0;
}

void cli_set_font16(font16* font)
{
    CURRENT_CLI_CTX.f16 = font;
}

void cli_set_color(u32 color)
{
    CURRENT_CLI_CTX.last_color = CURRENT_CLI_CTX.color;
    CURRENT_CLI_CTX.color = color;
}

void cli_set_bg_color(u32 color)
{
    CURRENT_CLI_CTX.last_background_color = CURRENT_CLI_CTX.background_color;
    CURRENT_CLI_CTX.background_color = color;
}

void cli_restore_color()
{
    CURRENT_CLI_CTX.color = CURRENT_CLI_CTX.last_color;
}

void cli_restore_bg_color()
{
    CURRENT_CLI_CTX.background_color = CURRENT_CLI_CTX.last_background_color;
}

int cli_write_char(char c)
{
    if (CURRENT_CLI_CTX.f16 == NULL) return 1;
    
    if (c == '\t')
    {
        for (int i = 0; i < 4; i++)
        {
            cli_write_char(' ');
        }
    } else if (c == '\n')
    {
        CURRENT_CLI_CTX.y += 16;
        CURRENT_CLI_CTX.cy++;
        if (CURRENT_CLI_CTX.y >= CURRENT_CLI_CTX.fb->height)
        {
            CURRENT_CLI_CTX.y = 0;
            CURRENT_CLI_CTX.cy = 0;
        }
    } else if (c == '\r')
    {
        CURRENT_CLI_CTX.x = 0;
        CURRENT_CLI_CTX.cx = 0;
    } else if (c == '\b')
    {
        if (CURRENT_CLI_CTX.cx > 0)
        {
            CURRENT_CLI_CTX.x -= 8;
            CURRENT_CLI_CTX.cx--;
        }
    } else {
        for (u32 fx = 0; fx < 8; fx++)
        {
            for (u32 fy = 0; fy < 16; fy++) {
                if (CURRENT_CLI_CTX.f16->letters[(u8)c].row[fy] & ((1 << (7 - fx))))
                {
                    framebuffer_put_pixel(CURRENT_CLI_CTX.fb, CURRENT_CLI_CTX.color, CURRENT_CLI_CTX.x + fx, CURRENT_CLI_CTX.y + fy);
                } else {
                    framebuffer_put_pixel(CURRENT_CLI_CTX.fb, CURRENT_CLI_CTX.background_color, CURRENT_CLI_CTX.x + fx, CURRENT_CLI_CTX.y + fy);
                }
            }
        }
    
        CURRENT_CLI_CTX.x += 8;
        CURRENT_CLI_CTX.cx++;
        if (CURRENT_CLI_CTX.x >= CURRENT_CLI_CTX.fb->width)
        {
            CURRENT_CLI_CTX.y += 16;
            CURRENT_CLI_CTX.x = 0;
            CURRENT_CLI_CTX.cx = 0;
        }
    
        if (CURRENT_CLI_CTX.y >= CURRENT_CLI_CTX.fb->height)
        {
            CURRENT_CLI_CTX.y = 0;
            CURRENT_CLI_CTX.cy = 0;
        }
    }

    return 0;
}

int cli_write_str(const char* s)
{
    for (size i = 0; s[i]; i++)
        if (cli_write_char(s[i]) == 1)
            return 1;

    return 0;
}