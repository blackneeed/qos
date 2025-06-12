#ifndef FONT_H
#define FONT_H
#include <types.h>

typedef struct {
    struct {
        u8 row[8];
    } letters[256];
} font8;

typedef struct {
    struct {
        u8 row[16];
    } letters[256];
} font16;
#endif