#ifndef PS2KB_H
#define PS2KB_H
#include <types.h>

static const char ps2kb_scancode_set2_map[] =
{
    0, 0, '1', '2',
    '3', '4', '5', '6',
    '7', '8', '9', '0',
    '-', '=', 0, 0,
    'q', 'w', 'e', 'r',
    't', 'y', 'u', 'i',
    'o', 'p', '[', ']',
    0, 0, 'a', 's',
    'd', 'f', 'g', 'h',
    'j', 'k', 'l', ';',
    '\'', '`', 0, '\\',
    'z', 'x', 'c', 'v',
    'b', 'n', 'm', ',',
    '.', '/', 0, '*',
    0, ' ', 0, 0,
    0, 0, 0, 0,
    0, 0, 0, 0,
    0, 0, 0
};

int ps2_set_scancode_set(u8 set);
void ps2_keyboard_interrupt();
#endif