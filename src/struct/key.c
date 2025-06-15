#include <struct/key.h>

char keycode_to_ascii(key_code code, u8 shift, u8 caps)
{
    if ((shift || caps) && !(shift && caps))
    {
        switch (code)
        {
            case KEY_a: return 'A'; break;
            case KEY_b: return 'B'; break;
            case KEY_c: return 'C'; break;
            case KEY_d: return 'D'; break;
            case KEY_e: return 'E'; break;
            case KEY_f: return 'F'; break;
            case KEY_g: return 'G'; break;
            case KEY_h: return 'H'; break;
            case KEY_i: return 'I'; break;
            case KEY_j: return 'J'; break;
            case KEY_k: return 'K'; break;
            case KEY_l: return 'L'; break;
            case KEY_m: return 'M'; break;
            case KEY_n: return 'N'; break;
            case KEY_o: return 'O'; break;
            case KEY_p: return 'P'; break;
            case KEY_q: return 'Q'; break;
            case KEY_r: return 'R'; break;
            case KEY_s: return 'S'; break;
            case KEY_t: return 'T'; break;
            case KEY_u: return 'U'; break;
            case KEY_v: return 'V'; break;
            case KEY_w: return 'W'; break;
            case KEY_x: return 'X'; break;
            case KEY_y: return 'Y'; break;
            case KEY_z: return 'Z'; break;
            default: break;
        }
    }
    
    if (shift)
    {
        switch (code)
        {
            case KEY_n0: return ')'; break;
            case KEY_n1: return '!'; break;
            case KEY_n2: return '@'; break;
            case KEY_n3: return '#'; break;
            case KEY_n4: return '$'; break;
            case KEY_n5: return '%'; break;
            case KEY_n6: return '^'; break;
            case KEY_n7: return '&'; break;
            case KEY_n8: return '*'; break;
            case KEY_n9: return '('; break;
            case KEY_backtick: return '~'; break;
            case KEY_minus: return '_'; break;
            case KEY_equals: return '+'; break;
            case KEY_backslash: return '|'; break;
            case KEY_brackl: return '{'; break;
            case KEY_brackr: return '}'; break;
            case KEY_semicolon: return ':'; break;
            case KEY_apostrophe: return '"'; break;
            case KEY_comma: return '<'; break;
            case KEY_dot: return '>'; break;
            case KEY_slash: return '?'; break;
            default: break;
        }
    }
    
    switch (code)
    {
        case KEY_a: return 'a'; break;
        case KEY_b: return 'b'; break;
        case KEY_c: return 'c'; break;
        case KEY_d: return 'd'; break;
        case KEY_e: return 'e'; break;
        case KEY_f: return 'f'; break;
        case KEY_g: return 'g'; break;
        case KEY_h: return 'h'; break;
        case KEY_i: return 'i'; break;
        case KEY_j: return 'j'; break;
        case KEY_k: return 'k'; break;
        case KEY_l: return 'l'; break;
        case KEY_m: return 'm'; break;
        case KEY_n: return 'n'; break;
        case KEY_o: return 'o'; break;
        case KEY_p: return 'p'; break;
        case KEY_q: return 'q'; break;
        case KEY_r: return 'r'; break;
        case KEY_s: return 's'; break;
        case KEY_t: return 't'; break;
        case KEY_u: return 'u'; break;
        case KEY_v: return 'v'; break;
        case KEY_w: return 'w'; break;
        case KEY_x: return 'x'; break;
        case KEY_y: return 'y'; break;
        case KEY_z: return 'z'; break;
        case KEY_n0: return '0'; break;
        case KEY_n1: return '1'; break;
        case KEY_n2: return '2'; break;
        case KEY_n3: return '3'; break;
        case KEY_n4: return '4'; break;
        case KEY_n5: return '5'; break;
        case KEY_n6: return '6'; break;
        case KEY_n7: return '7'; break;
        case KEY_n8: return '8'; break;
        case KEY_n9: return '9'; break;
        case KEY_backtick: return '`'; break;
        case KEY_minus: return '-'; break;
        case KEY_equals: return '='; break;
        case KEY_backslash: return '\\'; break;
        case KEY_backspace: return '\b'; break;
        case KEY_space: return ' '; break;
        case KEY_tab: return '\t'; break;
        case KEY_brackl: return '['; break;
        case KEY_numpad_slash: return '/'; break;
        case KEY_numpad_asterrisk: return '*'; break;
        case KEY_numpad_minus: return '-'; break;
        case KEY_numpad_plus: return '+'; break;
        case KEY_numpad_dot: return '.'; break;
        case KEY_numpad_0: return '0'; break;
        case KEY_numpad_1: return '1'; break;
        case KEY_numpad_2: return '2'; break;
        case KEY_numpad_3: return '3'; break;
        case KEY_numpad_4: return '4'; break;
        case KEY_numpad_5: return '5'; break;
        case KEY_numpad_6: return '6'; break;
        case KEY_numpad_7: return '7'; break;
        case KEY_numpad_8: return '8'; break;
        case KEY_numpad_9: return '9'; break;
        case KEY_brackr: return ']'; break;
        case KEY_semicolon: return ';'; break;
        case KEY_apostrophe: return '\''; break;
        case KEY_comma: return ','; break;
        case KEY_dot: return '.'; break;
        case KEY_slash: return '/'; break;
        default: return '\0'; break;
    }
}