#ifndef CLI_H
#define CLI_H
#include <types.h>
#include <struct/fb.h>
#include <struct/font.h>

void cli_init(framebuffer* fb);
void cli_set_font8(font8* font);
void cli_set_font16(font16* font);
void cli_set_color(u32 color);
void cli_set_bg_color(u32 color);
void cli_put_char(char c);
int cli_put_str(const char* s);
#endif