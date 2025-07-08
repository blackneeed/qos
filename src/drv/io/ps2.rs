use crate::tables::idt::register_irq;

const PS2_DATA_PORT: u16 = 0x60;
const PS2_STATUS_PORT: u16 = 0x64;
const PS2_TIMEOUT: u32 = 100000;
const PS2_RETRY_COUNT: u32 = 3;
const PS2_COMMAND_PORT: u16 = 0x64;
const PS2_COMMAND_DISABLE_PORT1: u8 = 0xAD;
const PS2_COMMAND_DISABLE_PORT2: u8 = 0xA7;
const PS2_COMMAND_ENABLE_PORT1: u8 = 0xAE;
const PS2_COMMAND_ENABLE_PORT2: u8 = 0xA8;
const PS2_COMMAND_READ_CONFIG: u8 = 0x20;
const PS2_COMMAND_WRITE_CONFIG: u8 = 0x60;
const PS2_COMMAND_READ_CTRL_OUT_PORT: u8 = 0xD0;
const PS2_COMMAND_WRITE_CTRL_OUT_PORT: u8 = 0xD1;
const PS2_COMMAND_PORT2_REDIR: u8 = 0xD4;
const PS2_COMMAND_TEST: u8 = 0xAA;
const PS2_COMMAND_TEST_PORT1: u8 = 0xAB;
const PS2_COMMAND_TEST_PORT2: u8 = 0xA9;
const PS2_DEVICE_COMMAND_RESET: u8 = 0xFF;
const PS2_DEVICE_COMMAND_DISABLE_SCANNING: u8 = 0xF5;
const PS2_DEVICE_COMMAND_ENABLE_SCANNING: u8 = 0xF4;
const PS2_DEVICE_COMMAND_IDENTIFY: u8 = 0xF2;
const PS2_TEST_SUCCESS: u8 = 0x55;
const PS2_TEST_PORT1_SUCCESS: u8 = 0x00;
const PS2_TEST_PORT2_SUCCESS: u8 = 0x00;

pub enum PS2Device {
    KeyboardAT,
    Mouse,
    MouseWithScrollWheel,
    Mouse5Buttons,
    KeyboardMF2,
    KeyboardShort,
    Keyboard122KeyHostConnected, // & ncd n97
    Keyboard122Key,
    KeyboardJapanG,
    KeyboardJapanP,
    KeyboardJapanA,
    KeyboardNCDSun,
    Unknown
}

pub struct PS2Controller;
impl PS2Controller {
    pub const fn new() {
        fn input_buf_clear() -> bool {
            false
        }

        fn output_buf_clear() -> bool {
            true
        }

        fn wait_input_buffer_clear() -> Result<(), ()> {
            Err(())
        }

        fn wait_output_buffer_full() -> Result<(), ()> {
            Err(())
        }

        fn send_command(command: u8) -> Result<(), ()> {
            Err(())
        }

        fn get_config_byte() -> Result<u8, ()> {
            Err(())
        }

        fn set_config_byte(byte: u8) -> Result<(), ()> {
            Err(())
        }

        fn send_port1(byte: u8) -> Result<(), ()> {
            Err(())
        }

        fn send_port2(byte: u8) -> Result<(), ()> {
            Err(())
        }

        fn test(command: u8, expected: u8) -> Result<(), ()> {
            Err(())
        }

        fn flush() {

        }

        fn get_type(send_byte: fn(u8)) -> PS2Device {
            PS2Device::Unknown
        }

        unimplemented!("ps2 initialization");
    }
}

unsafe fn ps2_port1_irq() {
    unimplemented!("ps2 irq1");
}

unsafe fn ps2_port2_irq() {
    unimplemented!("ps2 irq2");
}

pub unsafe fn ps2_init() {
    register_irq(1, || unsafe { ps2_port1_irq(); });
    register_irq(12, || unsafe { ps2_port2_irq(); });
}