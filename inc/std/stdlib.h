#ifndef STDLIB_H
#define STDLIB_H
#define UNUSED(x) (void)(x)
#define GET_BIT(v, n) ((v & (1 << n)) >> n)
#define MOD_BIT(v, n, nv) (((v) & ~(1 << (n))) | ((nv) << (n)))
#endif