#include <types.h>
#include <drv/ps2ctrl.h>
#include <drv/ps2kb.h>
#include <drv/ioport.h>
#include <std/stdio.h>
#include <std/stdlib.h>

static ps2ctrl_info CURRENT_PS2_CTX;

u8 ps2ctrl_read_status()
{
    io_wait();
    int rv = io_inb(PS2CTRL_STATUS_PORT);
    io_wait();
    return rv;
}

u8 ps2ctrl_input_buffer_clear()
{
    return GET_BIT(ps2ctrl_read_status(), PS2CTRL_STATUS_INPUT_BUFFER_STATUS_BIT_IDX) == 0;
}

u8 ps2ctrl_output_buffer_clear()
{
    return GET_BIT(ps2ctrl_read_status(), PS2CTRL_STATUS_OUTPUT_BUFFER_STATUS_BIT_IDX) == 0;
}

u8 ps2ctrl_wait_input_buffer_clear()
{
    for (size i = 0; i < PS2CTRL_GENERAL_TIMEOUT; i++)
        if (ps2ctrl_input_buffer_clear())
            return 1;

    kprintf("[PS2] ps2ctrl_wait_input_buffer_clear() timed out, max cycles: %d\r\n", PS2CTRL_GENERAL_TIMEOUT);
    return 0;
}

u8 ps2ctrl_wait_output_buffer_full(u8 error_expected)
{
    for (size i = 0; i < PS2CTRL_GENERAL_TIMEOUT; i++)
        if (!ps2ctrl_output_buffer_clear())
            return 1;
    
    if (!error_expected) kprintf("[PS2] ps2ctrl_wait_output_buffer_full() timed out, max cycles: %d\r\n", PS2CTRL_GENERAL_TIMEOUT);
    return 0;
}

u8 ps2ctrl_send_command(u8 command)
{
    for (size i = 0; i < PS2CTRL_GENERAL_RETRY_COUNT; i++)
    {
        if (!ps2ctrl_wait_input_buffer_clear())
        {
            kprintf("[PS2] ps2ctrl_send_command(): ps2ctrl_wait_input_buffer_clear() failed, retrying (try %d/%d)\r\n", i+1, PS2CTRL_GENERAL_RETRY_COUNT);
            continue;
        }

        io_outb(PS2CTRL_COMMAND_PORT, command);
        return 1;
    }
    return 0;
}

u8 ps2ctrl_get_config_byte(u8* out)
{
    if (!ps2ctrl_send_command(PS2CTRL_COMMAND_READ_CONFIG_BYTE)) return 0;
    if (!ps2ctrl_wait_output_buffer_full(0)) return 0;
    *out = io_inb(PS2CTRL_DATA_PORT);
    return 1;
}

u8 ps2ctrl_set_config_byte(u8 new)
{
    if (!ps2ctrl_send_command(PS2CTRL_COMMAND_WRITE_CONFIG_BYTE)) return 0;
    io_wait();
    io_outb(PS2CTRL_DATA_PORT, new);
    return 1;
}

u8 ps2ctrl_send_byte_port1(u8 byte)
{
    for (int i = 0; i < PS2CTRL_GENERAL_RETRY_COUNT; i++)
    {
        if (!ps2ctrl_wait_input_buffer_clear())
        {
            kprintf("[PS2] ps2ctrl_send_byte_port1(): ps2ctrl_wait_input_buffer_clear() failed, retrying (try %d/%d)\r\n", i+1, PS2CTRL_GENERAL_RETRY_COUNT);
            continue;
        }

        io_outb(PS2CTRL_DATA_PORT, byte);
        io_wait();

        if (!ps2ctrl_wait_output_buffer_full(0))
        {
            kprintf("[PS2] ps2ctrl_send_byte_port1(): ps2ctrl_wait_output_buffer_full() failed, retrying (try %d/%d)\r\n", i+1, PS2CTRL_GENERAL_RETRY_COUNT);
            continue;
        }
    
        u8 resp = io_inb(PS2CTRL_DATA_PORT);
        if (resp == 0xFA) return 1;

        kprintf("[PS2] ps2ctrl_send_byte_port1(): port 0x%X returned 0x%X, not ACK (0xFA) when sending byte 0x%X, retrying (try %d/%d)\r\n", PS2CTRL_DATA_PORT, resp, byte, i+1, PS2CTRL_GENERAL_RETRY_COUNT);
    }

    return 0;
}

u8 ps2ctrl_send_byte_port2(u8 byte)
{
    for (int i = 0; i < PS2CTRL_GENERAL_RETRY_COUNT; i++)
    {
        if (!ps2ctrl_wait_input_buffer_clear())
        {
            kprintf("[PS2] ps2ctrl_send_byte_port2(): ps2ctrl_wait_input_buffer_clear() failed, retrying (try %d/%d)\r\n", i+1, PS2CTRL_GENERAL_RETRY_COUNT);
            continue;
        }

        if (!ps2ctrl_send_command(PS2CTRL_COMMAND_NEXT_PORT2)) continue;
        if (!ps2ctrl_wait_input_buffer_clear())
        {
            kprintf("[PS2] ps2ctrl_send_byte_port2(): ps2ctrl_wait_input_buffer_clear() failed, retrying (try %d/%d)\r\n", i+1, PS2CTRL_GENERAL_RETRY_COUNT);
            continue;
        }
        
        io_outb(PS2CTRL_DATA_PORT, byte);
        io_wait();  
        if (!ps2ctrl_wait_output_buffer_full(0))
        {
            kprintf("[PS2] ps2ctrl_send_byte_port2(): ps2ctrl_wait_output_buffer_full() failed, retrying (max tries = %d)\r\n", PS2CTRL_GENERAL_RETRY_COUNT);
            continue;
        }
    
        u8 resp = io_inb(PS2CTRL_DATA_PORT);
        if (resp == 0xFA) return 1;
        kprintf("[PS2] ps2ctrl_send_byte_port2(): port 0x%X returned 0x%X, not ACK (0xFA) when sending byte 0x%X, retrying (try %d/%d)\r\n", PS2CTRL_DATA_PORT, resp, byte, i+1, PS2CTRL_GENERAL_RETRY_COUNT);
        continue;
    }

    return 0;
}

u8 ps2ctrl_perform_test(u8 test_command, u8 valid_return)
{
    if (!ps2ctrl_send_command(test_command)) return 0;
    ps2ctrl_wait_output_buffer_full(0);
    io_wait();
    if (io_inb(PS2CTRL_DATA_PORT) != valid_return) return 0;
    return 1;
}

u8 ps2ctrl_get_type(u8 (*send_byte)(u8), u8 type[], u8* type_size) // [2]
{
    if (!send_byte(PS2CTRL_DEVICE_COMMAND_RESET)) return 0;
    ps2ctrl_flush();  // flush - usually should only be 0xAA but only god knows what it spits out fr, 0x0, sometimes 0x1 tf
    if (!send_byte(PS2CTRL_DEVICE_COMMAND_IDENTIFY)) return 0;
    *type_size = 0;

    if (ps2ctrl_wait_output_buffer_full(1))
    {
        io_wait();
        type[0] = io_inb(PS2CTRL_DATA_PORT);
        (*type_size)++;

        if (ps2ctrl_wait_output_buffer_full(1))
        {
            io_wait();
            type[1] = io_inb(PS2CTRL_DATA_PORT);
            (*type_size)++;
        }
    }

    return 1;
}

void ps2ctrl_flush()
{
    for (size i = 0; i < PS2CTRL_GENERAL_TIMEOUT; i++)
        if (!ps2ctrl_output_buffer_clear())
            UNUSED(io_inb(PS2CTRL_DATA_PORT));
}

void ps2ctrl_port1_irq()
{
    if (ps2ctrl_is_keyboard_type(CURRENT_PS2_CTX.port1_type))
    {
        ps2kb_keyboard_interrupt();
    } else
    ps2ctrl_flush(); // bla bla bla
}

void ps2ctrl_port2_irq()
{
    if (ps2ctrl_is_keyboard_type(CURRENT_PS2_CTX.port2_type))
    {
        ps2kb_keyboard_interrupt();
    } else
    ps2ctrl_flush(); // bla bla bla
}

u8 ps2ctrl_init(ps2ctrl_info* information)
{
    u8 port1_avail = 1;
    u8 port2_avail = 0;

    if (!ps2ctrl_send_command(PS2CTRL_COMMAND_DISABLE_PORT1)) return 0;
    if (!ps2ctrl_send_command(PS2CTRL_COMMAND_DISABLE_PORT2)) return 0;

    ps2ctrl_flush();

    u8 config_byte;
    if (!ps2ctrl_get_config_byte(&config_byte)) return 0;
    config_byte = MOD_BIT(config_byte, PS2CTRL_CONFIG_BYTE_PORT1_TRANSLATION, 0);
    if (!ps2ctrl_set_config_byte(config_byte)) return 0;

    if (!ps2ctrl_send_command(PS2CTRL_COMMAND_ENABLE_PORT2)) return 0;
    if (!ps2ctrl_get_config_byte(&config_byte)) return 0;
    if (GET_BIT(config_byte, PS2CTRL_CONFIG_BYTE_PORT2_CLK) == 0) 
    {
        port2_avail = 1;
        if (!ps2ctrl_send_command(PS2CTRL_COMMAND_DISABLE_PORT2)) return 0;
        config_byte = MOD_BIT(config_byte, PS2CTRL_CONFIG_BYTE_PORT2_INT, 0);
        if (!ps2ctrl_set_config_byte(config_byte)) return 0;
    }

    ps2ctrl_flush();

    ps2ctrl_perform_test(PS2CTRL_COMMAND_TEST, PS2CTRL_TEST_VALID);
    if (!ps2ctrl_perform_test(PS2CTRL_COMMAND_TEST_PORT1, PS2CTRL_TEST_PORT1_VALID)) port1_avail = 0;
    if (port2_avail) if (!ps2ctrl_perform_test(PS2CTRL_COMMAND_TEST_PORT2, PS2CTRL_TEST_PORT2_VALID)) port2_avail = 0;

    u8 type1[2];
    u8 type1_size;
    ps2ctrl_device_type type1_e = PS2DEV_unknown;

    if (port1_avail)
    {
        if (!ps2ctrl_send_command(PS2CTRL_COMMAND_ENABLE_PORT1)) return 0;
        else if (!ps2ctrl_send_byte_port1(PS2CTRL_DEVICE_COMMAND_DISABLE_SCANNING)) return 0;
        else if (!ps2ctrl_get_type(ps2ctrl_send_byte_port1, type1, &type1_size)) return 0;
        else if (type1_size == 0) type1_e = PS2DEV_keyboard_at;
        else if (type1_size == 1 && type1[0] == 0x00) type1_e = PS2DEV_mouse;
        else if (type1_size == 1 && type1[0] == 0x03) type1_e = PS2DEV_mouse_scroll_wheel;
        else if (type1_size == 1 && type1[0] == 0x04) type1_e = PS2DEV_mouse_5_button;
        else if (type1_size == 2 && type1[0] == 0xAB && (type1[1] == 0x83 || type1[1] == 0xC1)) type1_e = PS2DEV_keyboard_motherfucker2;
        else if (type1_size == 2 && type1[0] == 0xAB && type1[1] == 0x84) type1_e = PS2DEV_keyboard_short;
        else if (type1_size == 2 && type1[0] == 0xAB && type1[1] == 0x85) type1_e = PS2DEV_keyboard_122_key_host_connected;
        else if (type1_size == 2 && type1[0] == 0xAB && type1[1] == 0x86) type1_e = PS2DEV_keyboard_122_key;
        else if (type1_size == 2 && type1[0] == 0xAB && type1[1] == 0x90) type1_e = PS2DEV_keyboard_japan_g;
        else if (type1_size == 2 && type1[0] == 0xAB && type1[1] == 0x91) type1_e = PS2DEV_keyboard_japan_p;
        else if (type1_size == 2 && type1[0] == 0xAB && type1[1] == 0x92) type1_e = PS2DEV_keyboard_japan_a;
        else if (type1_size == 2 && type1[0] == 0xAB && type1[1] == 0xA1) type1_e = PS2DEV_keyboard_ncd_sun;
        else type1_e = PS2DEV_unknown;
    }

    u8 type2[2];
    u8 type2_size;
    ps2ctrl_device_type type2_e = PS2DEV_unknown;

    if (port2_avail)
    {
        if (!ps2ctrl_send_command(PS2CTRL_COMMAND_ENABLE_PORT2)) return 0;
        if (!ps2ctrl_send_byte_port2(PS2CTRL_DEVICE_COMMAND_DISABLE_SCANNING)) return 0;
        if (!ps2ctrl_get_type(ps2ctrl_send_byte_port2, type2, &type2_size)) return 0;
        else if (type2_size == 0) type2_e = PS2DEV_keyboard_at;
        else if (type2_size == 1 && type2[0] == 0x00) type2_e = PS2DEV_mouse;
        else if (type2_size == 1 && type2[0] == 0x03) type2_e = PS2DEV_mouse_scroll_wheel;
        else if (type2_size == 1 && type2[0] == 0x04) type2_e = PS2DEV_mouse_5_button;
        else if (type2_size == 2 && type2[0] == 0xAB && (type2[1] == 0x83 || type2[1] == 0xC1)) type2_e = PS2DEV_keyboard_motherfucker2;
        else if (type2_size == 2 && type2[0] == 0xAB && type2[1] == 0x84) type2_e = PS2DEV_keyboard_short;
        else if (type2_size == 2 && type2[0] == 0xAB && type2[1] == 0x85) type2_e = PS2DEV_keyboard_122_key_host_connected;
        else if (type2_size == 2 && type2[0] == 0xAB && type2[1] == 0x86) type2_e = PS2DEV_keyboard_122_key;
        else if (type2_size == 2 && type2[0] == 0xAB && type2[1] == 0x90) type2_e = PS2DEV_keyboard_japan_g;
        else if (type2_size == 2 && type2[0] == 0xAB && type2[1] == 0x91) type2_e = PS2DEV_keyboard_japan_p;
        else if (type2_size == 2 && type2[0] == 0xAB && type2[1] == 0x92) type2_e = PS2DEV_keyboard_japan_a;
        else if (type2_size == 2 && type2[0] == 0xAB && type2[1] == 0xA1) type2_e = PS2DEV_keyboard_ncd_sun;
        else type2_e = PS2DEV_unknown;
    }

    if (!ps2ctrl_get_config_byte(&config_byte)) return 0;
    
    if (port1_avail) 
    {
        config_byte = MOD_BIT(config_byte, PS2CTRL_CONFIG_BYTE_PORT1_INT, 1);
        if (!ps2ctrl_send_byte_port1(PS2CTRL_DEVICE_COMMAND_ENABLE_SCANNING)) return 0;
    }

    if (port2_avail)
    {
        config_byte = MOD_BIT(config_byte, PS2CTRL_CONFIG_BYTE_PORT2_INT, 1);
        if (!ps2ctrl_send_byte_port2(PS2CTRL_DEVICE_COMMAND_ENABLE_SCANNING)) return 0;
    }

    if (!ps2ctrl_set_config_byte(config_byte)) return 0;

    CURRENT_PS2_CTX.port1_available = port1_avail;
    CURRENT_PS2_CTX.port2_available = port2_avail;
    CURRENT_PS2_CTX.port1_type = type1_e;
    CURRENT_PS2_CTX.port2_type = type2_e;
    *information = CURRENT_PS2_CTX;

    return 1;
}