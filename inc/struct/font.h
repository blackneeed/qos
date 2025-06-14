#ifndef FONT_H
#define FONT_H
#include <types.h>

typedef struct font16 {
    struct {
        u8 row[16];
    } letters[256];
} font16;
#endif