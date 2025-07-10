use crate::drv::io::ioport::{inb, outb};
use crate::drv::io::ps2kb::ps2_keyboard_irq;
use crate::tables::idt::register_irq;
use crate::{initialization_fail, initialized};
use spin::Mutex;

pub const PS2_DATA_PORT: u16 = 0x60;
const PS2_STATUS_PORT: u16 = 0x64;
const PS2_TIMEOUT: u32 = 100000;
const PS2_RETRY_COUNT: u32 = 3;
const PS2_COMMAND_PORT: u16 = 0x64;
const PS2_COMMAND_DISABLE_PORT1: u8 = 0xAD;
const PS2_COMMAND_ENABLE_PORT1: u8 = 0xAE;
const PS2_COMMAND_READ_CONFIG: u8 = 0x20;
const PS2_COMMAND_WRITE_CONFIG: u8 = 0x60;
const PS2_COMMAND_TEST: u8 = 0xAA;
const PS2_COMMAND_TEST_PORT1: u8 = 0xAB;
const PS2_DEVICE_COMMAND_RESET: u8 = 0xFF;
const PS2_DEVICE_COMMAND_DISABLE_SCANNING: u8 = 0xF5;
const PS2_DEVICE_COMMAND_ENABLE_SCANNING: u8 = 0xF4;
const PS2_DEVICE_COMMAND_IDENTIFY: u8 = 0xF2;
const PS2_TEST_SUCCESS: u8 = 0x55;
const PS2_TEST_PORT1_SUCCESS: u8 = 0x00;

static IGNORE_IRQ: Mutex<bool> = Mutex::new(false);

#[derive(Debug, Clone, PartialEq)]
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
    Unknown,
}

pub type PS2DeviceSend = fn(u8) -> Result<(), ()>;
static TYPE: Mutex<Option<PS2Device>> = Mutex::new(None);

unsafe fn ps2_port1_irq() {
    let mut lock = IGNORE_IRQ.lock();
    if *lock {
        inb(PS2_DATA_PORT);
        *lock = false;
        return;
    }

    assert!(TYPE.lock().is_some());
    if [
        PS2Device::KeyboardAT,
        PS2Device::KeyboardMF2,
        PS2Device::KeyboardShort,
        PS2Device::Keyboard122KeyHostConnected,
        PS2Device::Keyboard122Key,
        PS2Device::KeyboardNCDSun,
    ]
    .contains(&TYPE.lock().clone().unwrap())
    {
        ps2_keyboard_irq();
    } else {
        inb(PS2_DATA_PORT);
    }
}

fn input_buf_clear() -> bool {
    unsafe { (inb(PS2_STATUS_PORT) & (1 << 1)) == 0 }
}

fn output_buf_clear() -> bool {
    unsafe { (inb(PS2_STATUS_PORT) & (1 << 0)) == 0 }
}

fn wait_input_buffer_clear() -> Result<(), ()> {
    for _ in 0..PS2_TIMEOUT {
        if input_buf_clear() {
            return Ok(());
        }
    }

    Err(())
}

fn wait_output_buffer_full() -> Result<(), ()> {
    for _ in 0..PS2_TIMEOUT {
        if !output_buf_clear() {
            return Ok(());
        }
    }

    Err(())
}

fn send_command(command: u8) -> Result<(), ()> {
    for _ in 0..PS2_RETRY_COUNT {
        if wait_input_buffer_clear().is_ok() {
            unsafe {
                outb(PS2_COMMAND_PORT, command);
            }
            return Ok(());
        }
    }

    Err(())
}

fn get_config_byte() -> Result<u8, ()> {
    if send_command(PS2_COMMAND_READ_CONFIG).is_err() || wait_output_buffer_full().is_err() {
        Err(())
    } else {
        Ok(unsafe { inb(PS2_DATA_PORT) })
    }
}

fn set_config_byte(byte: u8) -> Result<(), ()> {
    if send_command(PS2_COMMAND_WRITE_CONFIG).is_ok() {
        unsafe {
            outb(PS2_DATA_PORT, byte);
        }
        return Ok(());
    }

    Err(())
}

fn send_port1(byte: u8) -> Result<(), ()> {
    for _ in 0..PS2_RETRY_COUNT {
        if wait_input_buffer_clear().is_err() {
            continue;
        }

        if byte == PS2_DEVICE_COMMAND_ENABLE_SCANNING {
            *IGNORE_IRQ.lock() = true;
        }

        unsafe {
            outb(PS2_DATA_PORT, byte);
        }

        if byte != PS2_DEVICE_COMMAND_ENABLE_SCANNING {
            if wait_output_buffer_full().is_err() {
                continue;
            }
            if unsafe { inb(PS2_DATA_PORT) } != 0xFA {
                continue;
            }
        }
        return Ok(());
    }

    if byte == PS2_DEVICE_COMMAND_ENABLE_SCANNING {
        Ok(())
    } else {
        Err(())
    }
}

fn test(command: u8, expected: u8) -> Result<(), ()> {
    if send_command(command).is_err() {
        return Err(());
    }

    if wait_output_buffer_full().is_err() {
        return Err(());
    };

    if unsafe { inb(PS2_DATA_PORT) } != expected {
        return Err(());
    }

    Ok(())
}

fn flush() {
    for _ in 0..PS2_TIMEOUT {
        if !output_buf_clear() {
            unsafe {
                inb(PS2_DATA_PORT);
            }
        }
    }
}

fn get_type(send_byte: PS2DeviceSend) -> Result<PS2Device, ()> {
    if send_byte(PS2_DEVICE_COMMAND_RESET).is_err() {
        return Err(());
    }
    flush();
    if send_byte(PS2_DEVICE_COMMAND_IDENTIFY).is_err() {
        return Err(());
    }

    let mut buf: [u8; 2] = [0; 2];
    let mut count: usize = 0;

    if wait_output_buffer_full().is_ok() {
        buf[count] = unsafe { inb(PS2_DATA_PORT) };
        count += 1;
        if wait_output_buffer_full().is_ok() {
            buf[count] = unsafe { inb(PS2_DATA_PORT) };
            count += 1;
        }
    }

    Ok(match count {
        0 => PS2Device::KeyboardAT,
        1 => match buf[0] {
            0x00 => PS2Device::Mouse,
            0x03 => PS2Device::MouseWithScrollWheel,
            0x04 => PS2Device::Mouse5Buttons,
            _ => PS2Device::Unknown,
        },
        2 => match buf[1] {
            0x83 | 0xC1 => PS2Device::KeyboardMF2,
            0x84 => PS2Device::KeyboardShort,
            0x85 => PS2Device::Keyboard122KeyHostConnected,
            0x86 => PS2Device::Keyboard122Key,
            0x90 => PS2Device::KeyboardJapanG,
            0x91 => PS2Device::KeyboardJapanP,
            0x92 => PS2Device::KeyboardJapanA,
            0xA1 => PS2Device::KeyboardNCDSun,
            _ => PS2Device::Unknown,
        },
        _ => PS2Device::Unknown,
    })
}

pub unsafe fn ps2_init() {
    let mut port1_available: bool = true;

    if send_command(PS2_COMMAND_DISABLE_PORT1).is_err() {
        initialization_fail!("PS2");
        return;
    }

    flush();

    if let Ok(cf) = get_config_byte()
        && set_config_byte(cf & !(1 << 6)).is_ok()
    {
    } else {
        initialization_fail!("PS2");
        return;
    }

    flush();
    if test(PS2_COMMAND_TEST, PS2_TEST_SUCCESS).is_err() {
        initialization_fail!("PS2");
        return;
    }

    if test(PS2_COMMAND_TEST_PORT1, PS2_TEST_PORT1_SUCCESS).is_err() {
        port1_available = false;
    }

    if let Ok(mut cf) = get_config_byte() {
        if port1_available {
            if send_command(PS2_COMMAND_ENABLE_PORT1).is_err() {
                initialization_fail!("PS2");
                return;
            }

            if send_port1(PS2_DEVICE_COMMAND_DISABLE_SCANNING).is_err() {
                initialization_fail!("PS2");
                return;
            }

            if let Ok(dev) = get_type(send_port1) {
                *TYPE.lock() = Some(dev);
                register_irq(1, || unsafe {
                    ps2_port1_irq();
                });

                if send_port1(PS2_DEVICE_COMMAND_ENABLE_SCANNING).is_err() {
                    initialization_fail!("PS2");
                    return;
                }

                cf &= !(1 << 4);
                cf |= 1 << 0;
            }
        }

        if set_config_byte(cf).is_err() {
            initialization_fail!("PS2");
            return;
        }
    } else {
        initialization_fail!("PS2");
        return;
    }

    initialized!("PS2");
}
