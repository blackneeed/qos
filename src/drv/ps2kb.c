#include <drv/ioport.h>
#include <drv/ps2kb.h>
#include <drv/ps2ctrl.h>
#include <struct/key.h>
#include <std/stdio.h>
#include <std/stdlib.h>
#include <nanoprintf.h>
#include <cli.h>

static struct {
    u8 initialized;
    key key_buf[PS2_INPUT_BUF_SIZE];
    size key_buf_count;
    u8 got_e0;
    u8 got_f0;
    u8 got_shift;
    u8 got_alt;
    u8 got_ctrl;
    u8 got_caps;
} CURRENT_PS2_CTX;

u8 ps2kb_init()
{
    CURRENT_PS2_CTX.initialized = 0;

    if (!ps2ctrl_send_byte_port1(0xF0)) return 0;
    if (!ps2ctrl_send_byte_port1(0x02)) return 0;

    CURRENT_PS2_CTX.key_buf_count = 0;
    CURRENT_PS2_CTX.got_e0 = 0;
    CURRENT_PS2_CTX.got_f0 = 0;
    CURRENT_PS2_CTX.got_shift = 0;
    CURRENT_PS2_CTX.got_alt = 0;
    CURRENT_PS2_CTX.got_ctrl = 0;
    CURRENT_PS2_CTX.got_caps = 0;

    CURRENT_PS2_CTX.initialized = 1;
    return 1;
}

key_code ps2kb_scancode_to_keycode(u8 scancode, u8 e0)
{
    if (e0)
    {
        switch (scancode)
        {
            case 0x1F: return lgui; break;
            case 0x14: return rctrl; break;
            case 0x27: return rgui; break;
            case 0x11: return ralt; break;
            case 0x2F: return apps; break;
            case 0x70: return insert; break;
            case 0x6C: return home; break;
            case 0x7D: return page_up; break;
            case 0x71: return del; break;
            case 0x69: return end; break;
            case 0x7A: return page_down; break;
            case 0x75: return up_arrow; break;
            case 0x6B: return left_arrow; break;
            case 0x72: return down_arrow; break;
            case 0x74: return right_arrow; break;
            case 0x4A: return numpad_slash; break;
            case 0x5A: return numpad_enter; break;
            case 0x37: return power; break;
            case 0x3F: return sleep; break;
            case 0x5E: return wake; break;
            case 0x4D: return next_track; break;
            case 0x15: return previous_track; break;
            case 0x3B: return stop; break;
            case 0x34: return play_pause; break;
            case 0x23: return mute; break;
            case 0x32: return vol_up; break;
            case 0x21: return vol_down; break;
            case 0x50: return media_select; break;
            case 0x48: return email; break;
            case 0x2B: return calc; break;
            case 0x40: return my_computer; break;
            case 0x10: return www_search; break;
            case 0x3A: return www_home; break;
            case 0x38: return www_back; break;
            case 0x30: return www_forward; break;
            case 0x28: return www_stop; break;
            case 0x20: return www_refresh; break;
            case 0x18: return www_favorites; break;
            default: return unknown; break;
        }
    } else {
        switch (scancode)
        {
            case 0x1C: return a; break;
            case 0x32: return b; break;
            case 0x21: return c; break;
            case 0x23: return d; break;
            case 0x24: return e; break;
            case 0x2B: return f; break;
            case 0x34: return g; break;
            case 0x33: return h; break;
            case 0x43: return i; break;
            case 0x3B: return j; break;
            case 0x42: return k; break;
            case 0x4B: return l; break;
            case 0x3A: return m; break;
            case 0x31: return n; break;
            case 0x44: return o; break;
            case 0x4D: return p; break;
            case 0x15: return q; break;
            case 0x2D: return r; break;
            case 0x1B: return s; break;
            case 0x2C: return t; break;
            case 0x3C: return u; break;
            case 0x2A: return v; break;
            case 0x1D: return w; break;
            case 0x22: return x; break;
            case 0x35: return y; break;
            case 0x1A: return z; break;
            case 0x45: return n0; break;
            case 0x16: return n1; break;
            case 0x1E: return n2; break;
            case 0x26: return n3; break;
            case 0x25: return n4; break;
            case 0x2E: return n5; break;
            case 0x36: return n6; break;
            case 0x3D: return n7; break;
            case 0x3E: return n8; break;
            case 0x46: return n9; break;
            case 0x0E: return backtick; break;
            case 0x4E: return minus; break;
            case 0x55: return equals; break;
            case 0x5D: return backslash; break;
            case 0x66: return backspace; break;
            case 0x29: return space; break;
            case 0x0D: return tab; break;
            case 0x58: return caps_lock; break;
            case 0x12: return lshift; break;
            case 0x14: return lctrl; break;
            case 0x11: return lalt; break;
            case 0x59: return rshift; break;
            case 0x5A: return enter; break;
            case 0x76: return escape; break;
            case 0x05: return f1; break;
            case 0x06: return f2; break;
            case 0x04: return f3; break;
            case 0x0C: return f4; break;
            case 0x03: return f5; break;
            case 0x0B: return f6; break;
            case 0x83: return f7; break;
            case 0x0A: return f8; break;
            case 0x01: return f9; break;
            case 0x09: return f10; break;
            case 0x78: return f11; break;
            case 0x07: return f12; break;
            case 0x7E: return scroll_lock; break;
            case 0x54: return brackl; break;
            case 0x77: return num_lock; break;
            case 0x7C: return numpad_asterrisk; break;
            case 0x7B: return numpad_minus; break;
            case 0x79: return numpad_plus; break;
            case 0x71: return numpad_dot; break;
            case 0x70: return numpad_0; break;
            case 0x69: return numpad_1; break;
            case 0x72: return numpad_2; break;
            case 0x7A: return numpad_3; break;
            case 0x6B: return numpad_4; break;
            case 0x73: return numpad_5; break;
            case 0x74: return numpad_6; break;
            case 0x6C: return numpad_7; break;
            case 0x75: return numpad_8; break;
            case 0x7D: return numpad_9; break;
            case 0x5B: return brackr; break;
            case 0x4C: return semicolon; break;
            case 0x52: return apostrophe; break;
            case 0x41: return comma; break;
            case 0x49: return dot; break;
            case 0x4A: return slash; break;
            default: return unknown; break;
        }
    }
}

void ps2kb_keyboard_interrupt()
{
    io_wait();
    u8 scancode = io_inb(0x60);
    io_wait();

    if (scancode == 0xE0)
    {
        CURRENT_PS2_CTX.got_e0 = 1;
    } else if (scancode == 0xF0)
    {
        CURRENT_PS2_CTX.got_f0 = 1;
    } else {
        key_code kc = ps2kb_scancode_to_keycode(scancode, CURRENT_PS2_CTX.got_e0);
        CURRENT_PS2_CTX.key_buf[CURRENT_PS2_CTX.key_buf_count].key = kc;
        CURRENT_PS2_CTX.key_buf[CURRENT_PS2_CTX.key_buf_count].ascii = keycode_to_ascii(kc, CURRENT_PS2_CTX.got_shift, CURRENT_PS2_CTX.got_caps);
        CURRENT_PS2_CTX.key_buf[CURRENT_PS2_CTX.key_buf_count].mods = (CURRENT_PS2_CTX.got_alt ? alt : 0) | (CURRENT_PS2_CTX.got_ctrl ? ctrl : 0) | (CURRENT_PS2_CTX.got_shift ? shift : 0) | (CURRENT_PS2_CTX.got_caps ? caps : 0);
        CURRENT_PS2_CTX.key_buf[CURRENT_PS2_CTX.key_buf_count].scancode = scancode;
        CURRENT_PS2_CTX.key_buf[CURRENT_PS2_CTX.key_buf_count].type = CURRENT_PS2_CTX.got_f0;
        CURRENT_PS2_CTX.key_buf_count++;
        if (kc == unknown)
        {
            kprintf("[PS2] unknown scancode %d\r\n", scancode);
        }

        if (kc == lshift || kc == rshift)
        {
            CURRENT_PS2_CTX.got_shift = CURRENT_PS2_CTX.got_f0 ? 0 : 1;
        } else if (kc == lctrl || kc == rctrl)
        {
            CURRENT_PS2_CTX.got_ctrl = CURRENT_PS2_CTX.got_f0 ? 0 : 1;
        } else if (kc == lalt || kc == ralt)
        {
            CURRENT_PS2_CTX.got_alt = CURRENT_PS2_CTX.got_f0 ? 0 : 1;
        } else if (kc == caps_lock)
        {
            CURRENT_PS2_CTX.got_caps = CURRENT_PS2_CTX.got_f0 ? 0 : 1;
        }

        CURRENT_PS2_CTX.got_e0 = 0;
        CURRENT_PS2_CTX.got_f0 = 0;
    }
}

u8 ps2kb_try_get_key(key* buf)
{
    if (CURRENT_PS2_CTX.key_buf_count <= 0) return 0;
    *buf = CURRENT_PS2_CTX.key_buf[CURRENT_PS2_CTX.key_buf_count - 1];
    CURRENT_PS2_CTX.key_buf_count--;
    return 1;
}