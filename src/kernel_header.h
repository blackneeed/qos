#pragma once
#include <drv/vga.h>

static vga_info* info;
void quickos_kernel_entry();
void quickos_kernel_loop();