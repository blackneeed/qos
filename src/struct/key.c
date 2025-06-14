#include <struct/key.h>

char keycode_to_ascii(key_code code, u8 shift, u8 caps)
{
    if ((shift || caps) && !(shift && caps))
    {
        switch (code)
        {
            case a: return 'A'; break;
            case b: return 'B'; break;
            case c: return 'C'; break;
            case d: return 'D'; break;
            case e: return 'E'; break;
            case f: return 'F'; break;
            case g: return 'G'; break;
            case h: return 'H'; break;
            case i: return 'I'; break;
            case j: return 'J'; break;
            case k: return 'K'; break;
            case l: return 'L'; break;
            case m: return 'M'; break;
            case n: return 'N'; break;
            case o: return 'O'; break;
            case p: return 'P'; break;
            case q: return 'Q'; break;
            case r: return 'R'; break;
            case s: return 'S'; break;
            case t: return 'T'; break;
            case u: return 'U'; break;
            case v: return 'V'; break;
            case w: return 'W'; break;
            case x: return 'X'; break;
            case y: return 'Y'; break;
            case z: return 'Z'; break;
            default: break;
        }
    }
    
    if (shift)
    {
        switch (code)
        {
            case n0: return ')'; break;
            case n1: return '!'; break;
            case n2: return '@'; break;
            case n3: return '#'; break;
            case n4: return '$'; break;
            case n5: return '%'; break;
            case n6: return '^'; break;
            case n7: return '&'; break;
            case n8: return '*'; break;
            case n9: return '('; break;
            case backtick: return '~'; break;
            case minus: return '_'; break;
            case equals: return '+'; break;
            case backslash: return '|'; break;
            case backspace: return '\b'; break;
            case space: return ' '; break;
            case tab: return '\t'; break;
            case brackl: return '{'; break;
            case numpad_slash: return '/'; break;
            case numpad_asterrisk: return '*'; break;
            case numpad_minus: return '-'; break;
            case numpad_plus: return '+'; break;
            case numpad_dot: return '.'; break;
            case numpad_0: return '0'; break;
            case numpad_1: return '1'; break;
            case numpad_2: return '2'; break;
            case numpad_3: return '3'; break;
            case numpad_4: return '4'; break;
            case numpad_5: return '5'; break;
            case numpad_6: return '6'; break;
            case numpad_7: return '7'; break;
            case numpad_8: return '8'; break;
            case numpad_9: return '9'; break;
            case brackr: return '}'; break;
            case semicolon: return ':'; break;
            case apostrophe: return '"'; break;
            case comma: return '<'; break;
            case dot: return '>'; break;
            case slash: return '?'; break;
            default: break;
        }
    }
    
    switch (code)
    {
        case a: return 'a'; break;
        case b: return 'b'; break;
        case c: return 'c'; break;
        case d: return 'd'; break;
        case e: return 'e'; break;
        case f: return 'f'; break;
        case g: return 'g'; break;
        case h: return 'h'; break;
        case i: return 'i'; break;
        case j: return 'j'; break;
        case k: return 'k'; break;
        case l: return 'l'; break;
        case m: return 'm'; break;
        case n: return 'n'; break;
        case o: return 'o'; break;
        case p: return 'p'; break;
        case q: return 'q'; break;
        case r: return 'r'; break;
        case s: return 's'; break;
        case t: return 't'; break;
        case u: return 'u'; break;
        case v: return 'v'; break;
        case w: return 'w'; break;
        case x: return 'x'; break;
        case y: return 'y'; break;
        case z: return 'z'; break;
        case n0: return '0'; break;
        case n1: return '1'; break;
        case n2: return '2'; break;
        case n3: return '3'; break;
        case n4: return '4'; break;
        case n5: return '5'; break;
        case n6: return '6'; break;
        case n7: return '7'; break;
        case n8: return '8'; break;
        case n9: return '9'; break;
        case backtick: return '`'; break;
        case minus: return '-'; break;
        case equals: return '='; break;
        case backslash: return '\\'; break;
        case backspace: return '\b'; break;
        case space: return ' '; break;
        case tab: return '\t'; break;
        case brackl: return '['; break;
        case numpad_slash: return '/'; break;
        case numpad_asterrisk: return '*'; break;
        case numpad_minus: return '-'; break;
        case numpad_plus: return '+'; break;
        case numpad_dot: return '.'; break;
        case numpad_0: return '0'; break;
        case numpad_1: return '1'; break;
        case numpad_2: return '2'; break;
        case numpad_3: return '3'; break;
        case numpad_4: return '4'; break;
        case numpad_5: return '5'; break;
        case numpad_6: return '6'; break;
        case numpad_7: return '7'; break;
        case numpad_8: return '8'; break;
        case numpad_9: return '9'; break;
        case brackr: return ']'; break;
        case semicolon: return ';'; break;
        case apostrophe: return '\''; break;
        case comma: return ','; break;
        case dot: return '.'; break;
        case slash: return '/'; break;
        default: return '\0'; break;
    }
}