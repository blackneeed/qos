use core::ptr::{read_volatile, write_volatile};

pub trait VolatileUnalignedOps {
    unsafe fn read_volatile_unaligned(ptr: *const Self) -> Self;
    unsafe fn write_volatile_unaligned(&self, ptr: *mut Self);
}

pub trait VolatileUnalignedPtrOps {
    type Target;

    unsafe fn read_volatile_unaligned(&self) -> Self::Target;
}

pub trait VolatileUnalignedMutPtrOps {
    type Target;

    unsafe fn read_volatile_unaligned(&self) -> Self::Target;
    unsafe fn write_volatile_unaligned(&self, value: Self::Target);
}

macro_rules! volatile_unaligned_impl {
    ($for:ty) => {
        impl VolatileUnalignedOps for $for {
            unsafe fn read_volatile_unaligned(ptr: *const $for) -> $for {
                let mut buf: [u8; core::mem::size_of::<$for>()] =
                    [0u8; core::mem::size_of::<$for>()];
                for (i, _) in buf.clone().iter().enumerate() {
                    buf[i] = read_volatile((ptr as *const u8).add(i));
                }

                *(buf.as_ptr() as *const $for)
            }

            unsafe fn write_volatile_unaligned(&self, ptr: *mut $for) {
                let mut idx: usize = 0;
                while idx < core::mem::size_of::<$for>() {
                    write_volatile(
                        (ptr as *mut u8).add(idx),
                        read_volatile((((&raw const *self) as *const u8).add(idx))),
                    );
                    idx += 1;
                }
            }
        }

        impl VolatileUnalignedPtrOps for *const $for {
            type Target = $for;

            unsafe fn read_volatile_unaligned(&self) -> Self::Target {
                <$for>::read_volatile_unaligned(*self)
            }
        }

        impl VolatileUnalignedMutPtrOps for *mut $for {
            type Target = $for;

            unsafe fn read_volatile_unaligned(&self) -> Self::Target {
                <$for>::read_volatile_unaligned(*self)
            }

            unsafe fn write_volatile_unaligned(&self, value: Self::Target) {
                value.write_volatile_unaligned(*self);
            }
        }
    };
}

volatile_unaligned_impl!(i8);
volatile_unaligned_impl!(u8);
volatile_unaligned_impl!(i16);
volatile_unaligned_impl!(u16);
volatile_unaligned_impl!(i32);
volatile_unaligned_impl!(u32);
volatile_unaligned_impl!(i64);
volatile_unaligned_impl!(u64);
volatile_unaligned_impl!(i128);
volatile_unaligned_impl!(u128);
volatile_unaligned_impl!(isize);
volatile_unaligned_impl!(usize);
