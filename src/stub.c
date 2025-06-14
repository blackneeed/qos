#include "types.h"

u64 __udivdi3(u64 dividend, u64 divisor) {
    if (divisor == 0) return 0;
    u64 quotient = 0, remainder = 0;
    for (int i = 63; i >= 0; i--) {
        remainder = (remainder << 1) | ((dividend >> i) & 1);
        if (remainder >= divisor) {
            remainder -= divisor;
            quotient |= ((u64)1 << i);
        }
    }
    return quotient;
}

u64 __umoddi3(u64 dividend, u64 divisor) {
    if (divisor == 0) return 0;
    u64 remainder = 0;
    for (int i = 63; i >= 0; i--) {
        remainder = (remainder << 1) | ((dividend >> i) & 1);
        if (remainder >= divisor) remainder -= divisor;
    }
    return remainder;
}

i64 __divdi3(i64 a, i64 b) {
    if (b == 0) return 0;
    int neg = 0;
    if (a < 0) { a = -a; neg ^= 1; }
    if (b < 0) { b = -b; neg ^= 1; }

    u64 quot = __udivdi3((u64)a, (u64)b);
    return neg ? -(i64)quot : (i64)quot;
}

i64 __moddi3(i64 a, i64 b) {
    if (b == 0) return 0;
    int neg = 0;
    if (a < 0) { a = -a; neg = 1; }
    if (b < 0) b = -b;

    u64 rem = __umoddi3((u64)a, (u64)b);
    return neg ? -(i64)rem : (i64)rem;
}