#ifndef PS2CTRL_H
#define PS2CTRL_H
#include <types.h>
#define PS2CTRL_DATA_PORT 0x60

#define PS2CTRL_STATUS_PORT 0x64
#define PS2CTRL_STATUS_OUTPUT_BUFFER_STATUS_BIT_IDX 0
#define PS2CTRL_STATUS_INPUT_BUFFER_STATUS_BIT_IDX 1
#define PS2CTRL_STATUS_SYSTEM_FLAG_BIT_IDX 2
#define PS2CTRL_STATUS_TIMEOUT_ERROR_BIT_IDX 6
#define PS2CTRL_STATUS_PARITY_ERROR_BIT_IDX 7

#define PS2CTRL_GENERAL_TIMEOUT 100000 // real hw rapes ts
#define PS2CTRL_GENERAL_RETRY_COUNT 3

#define PS2CTRL_COMMAND_PORT 0x64
#define PS2CTRL_COMMAND_DISABLE_PORT1 0xAD
#define PS2CTRL_COMMAND_ENABLE_PORT1 0xAE
#define PS2CTRL_COMMAND_DISABLE_PORT2 0xA7
#define PS2CTRL_COMMAND_ENABLE_PORT2 0xA8
#define PS2CTRL_COMMAND_READ_CONFIG_BYTE 0x20
#define PS2CTRL_COMMAND_WRITE_CONFIG_BYTE 0x60
#define PS2CTRL_COMMAND_READ_CONTROLLER_OUTPUT_PORT 0xD0
#define PS2CTRL_COMMAND_WRITE_CONTROLLER_OUTPUT_PORT 0xD1
#define PS2CTRL_COMMAND_NEXT_PORT2 0xD4
#define PS2CTRL_COMMAND_TEST 0xAA
#define PS2CTRL_COMMAND_TEST_PORT1 0xAB
#define PS2CTRL_COMMAND_TEST_PORT2 0xA9

#define PS2CTRL_DEVICE_COMMAND_RESET 0xFF
#define PS2CTRL_DEVICE_COMMAND_DISABLE_SCANNING 0xF5
#define PS2CTRL_DEVICE_COMMAND_ENABLE_SCANNING 0xF4
#define PS2CTRL_DEVICE_COMMAND_IDENTIFY 0xF2

#define PS2CTRL_TEST_VALID 0x55
#define PS2CTRL_TEST_PORT1_VALID 0x00
#define PS2CTRL_TEST_PORT2_VALID 0x00

#define PS2CTRL_CONFIG_BYTE_PORT1_INT 0
#define PS2CTRL_CONFIG_BYTE_PORT2_INT 1
#define PS2CTRL_CONFIG_BYTE_SYSTEM_FLAG 2
#define PS2CTRL_CONFIG_BYTE_PORT1_CLK 4
#define PS2CTRL_CONFIG_BYTE_PORT2_CLK 5
#define PS2CTRL_CONFIG_BYTE_PORT1_TRANSLATION 6

typedef enum ps2ctrl_device_type {
    PS2DEV_keyboard_at,
    PS2DEV_mouse,
    PS2DEV_mouse_scroll_wheel,
    PS2DEV_mouse_5_button,
    PS2DEV_keyboard_motherfucker2,
    PS2DEV_keyboard_short,
    PS2DEV_keyboard_122_key_host_connected, // ncd n97 shares the id
    PS2DEV_keyboard_122_key,
    PS2DEV_keyboard_japan_g,
    PS2DEV_keyboard_japan_p,
    PS2DEV_keyboard_japan_a,
    PS2DEV_keyboard_ncd_sun,
    PS2DEV_unknown
} ps2ctrl_device_type;

typedef struct ps2ctrl_info {
    u8 port1_available;
    u8 port2_available;
    ps2ctrl_device_type port1_type;
    ps2ctrl_device_type port2_type;
} ps2ctrl_info;

u8 ps2ctrl_read_status();
u8 ps2ctrl_input_buffer_clear();
u8 ps2ctrl_output_buffer_clear();
u8 ps2ctrl_wait_input_buffer_clear();
u8 ps2ctrl_wait_output_buffer_full(u8 error_expected);
u8 ps2ctrl_send_command(u8 command);
u8 ps2ctrl_get_config_byte(u8* out);
u8 ps2ctrl_set_config_byte(u8 new);
u8 ps2ctrl_send_byte_port1(u8 byte);
u8 ps2ctrl_send_byte_port2(u8 byte);
u8 ps2ctrl_perform_test(u8 test_command, u8 valid_return);
u8 ps2ctrl_get_type(u8 (*send_byte)(u8), u8 type[], u8* type_size);
void ps2ctrl_flush();
void ps2ctrl_port1_irq();
void ps2ctrl_port2_irq();
u8 ps2ctrl_init(ps2ctrl_info* information);

static inline u8 ps2ctrl_is_keyboard_type(ps2ctrl_device_type type)
{
    return type == PS2DEV_keyboard_122_key || type == PS2DEV_keyboard_122_key_host_connected || type == PS2DEV_keyboard_at || type == PS2DEV_keyboard_japan_a || type == PS2DEV_keyboard_japan_g || type == PS2DEV_keyboard_japan_p || type == PS2DEV_keyboard_motherfucker2 || type == PS2DEV_keyboard_ncd_sun || type == PS2DEV_keyboard_short;
}

static inline u8 ps2ctrl_is_mouse_type(ps2ctrl_device_type type)
{
    return type == PS2DEV_mouse || type == PS2DEV_mouse_5_button || type == PS2DEV_mouse_scroll_wheel;
}
#endif