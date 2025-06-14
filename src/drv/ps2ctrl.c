#include <types.h>
#include <drv/ps2ctrl.h>
#include <drv/ioport.h>
#include <std/stdio.h>
#include <std/stdlib.h>

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
    for (size i = 0; i < PS2CTRL_BUFFER_STATUS_TIMEOUT; i++)
        if (ps2ctrl_input_buffer_clear())
            return 1;

    kprintf("[PS2] ps2ctrl_wait_input_buffer_clear() timed out, max cycles: %d\r\n", PS2CTRL_BUFFER_STATUS_TIMEOUT);
    return 0;
}

u8 ps2ctrl_wait_output_buffer_full()
{
    for (size i = 0; i < PS2CTRL_BUFFER_STATUS_TIMEOUT; i++)
        if (!ps2ctrl_output_buffer_clear())
            return 1;
    
    kprintf("[PS2] ps2ctrl_wait_output_buffer_full() timed out, max cycles: %d\r\n", PS2CTRL_BUFFER_STATUS_TIMEOUT);
    return 0;
}

u8 ps2ctrl_send_command(u8 command)
{
    for (size i = 0; i < PS2CTRL_COMMAND_RETRY_COUNT; i++)
    {
        if (!ps2ctrl_wait_input_buffer_clear())
        {
            kprintf("[PS2] ps2ctrl_send_command(): ps2ctrl_wait_input_buffer_clear() failed, retrying (try %d/%d)\r\n", i+1, PS2CTRL_COMMAND_RETRY_COUNT);
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
    if (!ps2ctrl_wait_output_buffer_full()) return 0;
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
    for (int i = 0; i < PS2CTRL_COMMAND_RETRY_COUNT; i++)
    {
        if (!ps2ctrl_wait_input_buffer_clear())
        {
            kprintf("[PS2] ps2ctrl_send_byte_port1(): ps2ctrl_wait_input_buffer_clear() failed, retrying (try %d/%d)\r\n", i+1, PS2CTRL_COMMAND_RETRY_COUNT);
            continue;
        }

        io_outb(PS2CTRL_DATA_PORT, byte);
        io_wait();

        if (!ps2ctrl_wait_output_buffer_full())
        {
            kprintf("[PS2] ps2ctrl_send_byte_port1(): ps2ctrl_wait_output_buffer_full() failed, retrying (try %d/%d)\r\n", i+1, PS2CTRL_COMMAND_RETRY_COUNT);
            continue;
        }
    
        u8 resp = io_inb(PS2CTRL_DATA_PORT);
        if (resp == 0xFA) return 1;

        kprintf("[PS2] ps2ctrl_send_byte_port1(): port %d didn't return ACK (0xFA), retrying (try %d/%d)\r\n", PS2CTRL_DATA_PORT, i+1, PS2CTRL_COMMAND_RETRY_COUNT);
    }

    return 0;
}

u8 ps2ctrl_send_byte_port2(u8 byte)
{
    for (int i = 0; i < PS2CTRL_COMMAND_RETRY_COUNT; i++)
    {
        if (!ps2ctrl_wait_input_buffer_clear())
        {
            kprintf("[PS2] ps2ctrl_send_byte_port2(): ps2ctrl_wait_input_buffer_clear() failed, retrying (max tries = %d)\r\n", PS2CTRL_COMMAND_RETRY_COUNT);
            continue;
        }

        if (!ps2ctrl_send_command(PS2CTRL_COMMAND_NEXT_PORT2)) continue;
        if (!ps2ctrl_wait_input_buffer_clear())
        {
            kprintf("[PS2] ps2ctrl_send_byte_port2(): ps2ctrl_wait_input_buffer_clear() failed, retrying (max tries = %d)\r\n", PS2CTRL_COMMAND_RETRY_COUNT);
            continue;
        }
        
        io_outb(PS2CTRL_DATA_PORT, byte);
        io_wait();  
        if (!ps2ctrl_wait_output_buffer_full())
        {
            kprintf("[PS2] ps2ctrl_send_byte_port2(): ps2ctrl_wait_output_buffer_full() failed, retrying (max tries = %d)\r\n", PS2CTRL_COMMAND_RETRY_COUNT);
            continue;
        }
    
        u8 resp = io_inb(PS2CTRL_DATA_PORT);
        if (resp == 0xFA) return 1;
        kprintf("[PS2] ps2ctrl_send_byte_port2(): port %d didn't return ACK (0xFA), retrying (try %d/%d)\r\n", PS2CTRL_DATA_PORT, i+1, PS2CTRL_COMMAND_RETRY_COUNT);
        continue;
    }

    return 0;
}

u8 ps2ctrl_init(u8 port1_enable, u8 port2_enable)
{
    if (port1_enable > 0) port1_enable = 1;
    if (port2_enable > 0) port2_enable = 1;
    if (!ps2ctrl_send_command(PS2CTRL_COMMAND_DISABLE_PORT1)) return 0;

    if (!ps2ctrl_send_command(PS2CTRL_COMMAND_DISABLE_PORT2)) return 0;
    if (!ps2ctrl_output_buffer_clear()) UNUSED(io_inb(PS2CTRL_DATA_PORT));
    u8 config_byte;
    if (!ps2ctrl_get_config_byte(&config_byte)) return 0;
    config_byte = MOD_BIT(config_byte, PS2CTRL_CONFIG_BYTE_PORT1_INT, port1_enable);
    config_byte = MOD_BIT(config_byte, PS2CTRL_CONFIG_BYTE_PORT2_INT, port2_enable);
    config_byte = MOD_BIT(config_byte, PS2CTRL_CONFIG_BYTE_PORT1_CLK, ~port1_enable);
    config_byte = MOD_BIT(config_byte, PS2CTRL_CONFIG_BYTE_PORT2_CLK, ~port2_enable);
    config_byte = MOD_BIT(config_byte, PS2CTRL_CONFIG_BYTE_PORT1_TRANSLATION, 0);
    if (!ps2ctrl_set_config_byte(config_byte)) return 0;

    if (port1_enable) if (!ps2ctrl_send_command(PS2CTRL_COMMAND_ENABLE_PORT1)) return 0;
    if (port2_enable) if (!ps2ctrl_send_command(PS2CTRL_COMMAND_ENABLE_PORT2)) return 0;

    return 1;
}