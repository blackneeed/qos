#ifndef CLI_H
#define CLI_H
#include <types.h>
#include <struct/fb.h>
#include <struct/font.h>

void cli_init(framebuffer* fb);
void cli_set_font16(font16* font);
void cli_set_color(u32 color);
void cli_set_bg_color(u32 color);
void cli_restore_color();
void cli_restore_bg_color();
void cli_write_char(char c);
int cli_write_str(const char* s);
#endif