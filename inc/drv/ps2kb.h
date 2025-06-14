#ifndef PS2KB_H
#define PS2KB_H
#include <types.h>
#include <struct/key.h>

#define PS2_INPUT_BUF_SIZE 256

u8 ps2kb_init();
key_code ps2kb_scancode_to_keycode(u8 scancode, u8 e0);
void ps2kb_keyboard_interrupt();
u8 ps2kb_try_get_key(key* buf);
#endif