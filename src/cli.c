#include <types.h>
#include <struct/font.h>
#include <struct/fb.h>

static struct {
    u32 x;
    u32 y;
    framebuffer* fb;
    font8* f8;
    font16* f16;
    u8 f16_used;
    u32 color;
    u32 last_color;
    u32 background_color;
    u32 last_background_color;
} CURRENT_CLI_CTX;

void cli_init(framebuffer* fb)
{
    CURRENT_CLI_CTX.f8 = NULL;
    CURRENT_CLI_CTX.f16 = NULL;
    CURRENT_CLI_CTX.f16_used = 0;
    CURRENT_CLI_CTX.fb = fb;
    CURRENT_CLI_CTX.x = 0;
    CURRENT_CLI_CTX.y = 0;
    CURRENT_CLI_CTX.color = 0;
    CURRENT_CLI_CTX.last_color = 0;
    CURRENT_CLI_CTX.background_color = 0;
    CURRENT_CLI_CTX.last_background_color = 0;
}

void cli_set_font8(font8* font)
{
    CURRENT_CLI_CTX.f8 = font;
    CURRENT_CLI_CTX.f16_used = 0;
}

void cli_set_font16(font16* font)
{
    CURRENT_CLI_CTX.f16 = font;
    CURRENT_CLI_CTX.f16_used = 1;
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

int cli_put_char(char c)
{
    if ((!CURRENT_CLI_CTX.f16_used && CURRENT_CLI_CTX.f8 == NULL) || (CURRENT_CLI_CTX.f16_used && CURRENT_CLI_CTX.f16 == NULL)) return 1;
    u32 ys = CURRENT_CLI_CTX.f16_used ? 16 : 8;
    
    if (c == '\t')
    {
        for (int i = 0; i < 4; i++)
        {
            cli_put_char(' ');
        }
    } else if (c == '\n')
    {
        CURRENT_CLI_CTX.y += ys;
        if (CURRENT_CLI_CTX.y >= CURRENT_CLI_CTX.fb->height)
        {
            CURRENT_CLI_CTX.y = 0;
        }
    } else if (c == '\r')
    {
        CURRENT_CLI_CTX.x = 0;
    } else {
        for (u32 fx = 0; fx < 8; fx++)
        {
            for (u32 fy = 0; fy < ys; fy++) {
                if (CURRENT_CLI_CTX.f16_used)
                {
                    if (CURRENT_CLI_CTX.f16->letters[(u8)c].row[fy] & ((1 << (7 - fx))))
                    {
                        framebuffer_put_pixel(CURRENT_CLI_CTX.fb, CURRENT_CLI_CTX.color, CURRENT_CLI_CTX.x + fx, CURRENT_CLI_CTX.y + fy);
                    } else {
                        framebuffer_put_pixel(CURRENT_CLI_CTX.fb, CURRENT_CLI_CTX.background_color, CURRENT_CLI_CTX.x + fx, CURRENT_CLI_CTX.y + fy);
                    }
                } else {
                    if (CURRENT_CLI_CTX.f8->letters[(u8)c].row[fy] & ((1 << (7 - fx))))
                    {
                        framebuffer_put_pixel(CURRENT_CLI_CTX.fb, CURRENT_CLI_CTX.color, CURRENT_CLI_CTX.x + fx, CURRENT_CLI_CTX.y + fy);
                    } else {
                        framebuffer_put_pixel(CURRENT_CLI_CTX.fb, CURRENT_CLI_CTX.background_color, CURRENT_CLI_CTX.x + fx, CURRENT_CLI_CTX.y + fy);
                    }
                }
            }
        }
    
    
        CURRENT_CLI_CTX.x += 8;
        if (CURRENT_CLI_CTX.x >= CURRENT_CLI_CTX.fb->width)
        {
            CURRENT_CLI_CTX.y += ys;
            CURRENT_CLI_CTX.x = 0;
        }
    
        if (CURRENT_CLI_CTX.y >= CURRENT_CLI_CTX.fb->height)
        {
            CURRENT_CLI_CTX.y = 0;
        }
    }

    return 0;
}

int cli_put_str(const char* s)
{
    for (size i = 0; s[i]; i++)
        if (cli_put_char(s[i]) == 1)
            return 1;

    return 0;
}