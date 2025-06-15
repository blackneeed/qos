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
            case 0x1F: return KEY_lgui; break;
            case 0x14: return KEY_rctrl; break;
            case 0x27: return KEY_rgui; break;
            case 0x11: return KEY_ralt; break;
            case 0x2F: return KEY_apps; break;
            case 0x70: return KEY_insert; break;
            case 0x6C: return KEY_home; break;
            case 0x7D: return KEY_page_up; break;
            case 0x71: return KEY_del; break;
            case 0x69: return KEY_end; break;
            case 0x7A: return KEY_page_down; break;
            case 0x75: return KEY_up_arrow; break;
            case 0x6B: return KEY_left_arrow; break;
            case 0x72: return KEY_down_arrow; break;
            case 0x74: return KEY_right_arrow; break;
            case 0x4A: return KEY_numpad_slash; break;
            case 0x5A: return KEY_numpad_enter; break;
            case 0x37: return KEY_power; break;
            case 0x3F: return KEY_sleep; break;
            case 0x5E: return KEY_wake; break;
            case 0x4D: return KEY_next_track; break;
            case 0x15: return KEY_previous_track; break;
            case 0x3B: return KEY_stop; break;
            case 0x34: return KEY_play_pause; break;
            case 0x23: return KEY_mute; break;
            case 0x32: return KEY_vol_up; break;
            case 0x21: return KEY_vol_down; break;
            case 0x50: return KEY_media_select; break;
            case 0x48: return KEY_email; break;
            case 0x2B: return KEY_calc; break;
            case 0x40: return KEY_my_computer; break;
            case 0x10: return KEY_www_search; break;
            case 0x3A: return KEY_www_home; break;
            case 0x38: return KEY_www_back; break;
            case 0x30: return KEY_www_forward; break;
            case 0x28: return KEY_www_stop; break;
            case 0x20: return KEY_www_refresh; break;
            case 0x18: return KEY_www_favorites; break;
            default: return KEY_unknown; break;
        }
    } else {
        switch (scancode)
        {
            case 0x1C: return KEY_a; break;
            case 0x32: return KEY_b; break;
            case 0x21: return KEY_c; break;
            case 0x23: return KEY_d; break;
            case 0x24: return KEY_e; break;
            case 0x2B: return KEY_f; break;
            case 0x34: return KEY_g; break;
            case 0x33: return KEY_h; break;
            case 0x43: return KEY_i; break;
            case 0x3B: return KEY_j; break;
            case 0x42: return KEY_k; break;
            case 0x4B: return KEY_l; break;
            case 0x3A: return KEY_m; break;
            case 0x31: return KEY_n; break;
            case 0x44: return KEY_o; break;
            case 0x4D: return KEY_p; break;
            case 0x15: return KEY_q; break;
            case 0x2D: return KEY_r; break;
            case 0x1B: return KEY_s; break;
            case 0x2C: return KEY_t; break;
            case 0x3C: return KEY_u; break;
            case 0x2A: return KEY_v; break;
            case 0x1D: return KEY_w; break;
            case 0x22: return KEY_x; break;
            case 0x35: return KEY_y; break;
            case 0x1A: return KEY_z; break;
            case 0x45: return KEY_n0; break;
            case 0x16: return KEY_n1; break;
            case 0x1E: return KEY_n2; break;
            case 0x26: return KEY_n3; break;
            case 0x25: return KEY_n4; break;
            case 0x2E: return KEY_n5; break;
            case 0x36: return KEY_n6; break;
            case 0x3D: return KEY_n7; break;
            case 0x3E: return KEY_n8; break;
            case 0x46: return KEY_n9; break;
            case 0x0E: return KEY_backtick; break;
            case 0x4E: return KEY_minus; break;
            case 0x55: return KEY_equals; break;
            case 0x5D: return KEY_backslash; break;
            case 0x66: return KEY_backspace; break;
            case 0x29: return KEY_space; break;
            case 0x0D: return KEY_tab; break;
            case 0x58: return KEY_caps_lock; break;
            case 0x12: return KEY_lshift; break;
            case 0x14: return KEY_lctrl; break;
            case 0x11: return KEY_lalt; break;
            case 0x59: return KEY_rshift; break;
            case 0x5A: return KEY_enter; break;
            case 0x76: return KEY_escape; break;
            case 0x05: return KEY_f1; break;
            case 0x06: return KEY_f2; break;
            case 0x04: return KEY_f3; break;
            case 0x0C: return KEY_f4; break;
            case 0x03: return KEY_f5; break;
            case 0x0B: return KEY_f6; break;
            case 0x83: return KEY_f7; break;
            case 0x0A: return KEY_f8; break;
            case 0x01: return KEY_f9; break;
            case 0x09: return KEY_f10; break;
            case 0x78: return KEY_f11; break;
            case 0x07: return KEY_f12; break;
            case 0x7E: return KEY_scroll_lock; break;
            case 0x54: return KEY_brackl; break;
            case 0x77: return KEY_num_lock; break;
            case 0x7C: return KEY_numpad_asterrisk; break;
            case 0x7B: return KEY_numpad_minus; break;
            case 0x79: return KEY_numpad_plus; break;
            case 0x71: return KEY_numpad_dot; break;
            case 0x70: return KEY_numpad_0; break;
            case 0x69: return KEY_numpad_1; break;
            case 0x72: return KEY_numpad_2; break;
            case 0x7A: return KEY_numpad_3; break;
            case 0x6B: return KEY_numpad_4; break;
            case 0x73: return KEY_numpad_5; break;
            case 0x74: return KEY_numpad_6; break;
            case 0x6C: return KEY_numpad_7; break;
            case 0x75: return KEY_numpad_8; break;
            case 0x7D: return KEY_numpad_9; break;
            case 0x5B: return KEY_brackr; break;
            case 0x4C: return KEY_semicolon; break;
            case 0x52: return KEY_apostrophe; break;
            case 0x41: return KEY_comma; break;
            case 0x49: return KEY_dot; break;
            case 0x4A: return KEY_slash; break;
            default: return KEY_unknown; break;
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
        if (kc == KEY_unknown)
        {
            kprintf("[PS2] unknown scancode %d\r\n", scancode);
            return;
        }
        
        CURRENT_PS2_CTX.key_buf[CURRENT_PS2_CTX.key_buf_count].key = kc;
        CURRENT_PS2_CTX.key_buf[CURRENT_PS2_CTX.key_buf_count].ascii = keycode_to_ascii(kc, CURRENT_PS2_CTX.got_shift, CURRENT_PS2_CTX.got_caps);
        CURRENT_PS2_CTX.key_buf[CURRENT_PS2_CTX.key_buf_count].mods = (CURRENT_PS2_CTX.got_alt ? alt : 0) | (CURRENT_PS2_CTX.got_ctrl ? ctrl : 0) | (CURRENT_PS2_CTX.got_shift ? shift : 0) | (CURRENT_PS2_CTX.got_caps ? caps : 0);
        CURRENT_PS2_CTX.key_buf[CURRENT_PS2_CTX.key_buf_count].scancode = scancode;
        CURRENT_PS2_CTX.key_buf[CURRENT_PS2_CTX.key_buf_count].type = CURRENT_PS2_CTX.got_f0;
        CURRENT_PS2_CTX.key_buf_count++;

        if (kc == KEY_lshift || kc == KEY_rshift)
        {
            CURRENT_PS2_CTX.got_shift = CURRENT_PS2_CTX.got_f0 ? 0 : 1;
        } else if (kc == KEY_lctrl || kc == KEY_rctrl)
        {
            CURRENT_PS2_CTX.got_ctrl = CURRENT_PS2_CTX.got_f0 ? 0 : 1;
        } else if (kc == KEY_lalt || kc == KEY_ralt)
        {
            CURRENT_PS2_CTX.got_alt = CURRENT_PS2_CTX.got_f0 ? 0 : 1;
        } else if (kc == KEY_caps_lock)
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