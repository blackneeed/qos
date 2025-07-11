use core::any::type_name;
use core::convert::TryFrom;
use core::fmt::Debug;

#[derive(Debug)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

impl Range {
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn chop(self, chop: Range) -> ChopResult {
        let Range { start, end } = self;

        if chop.start >= end || chop.end <= start || chop.start >= chop.end {
            return ChopResult::One(Range::new(start, end));
        }

        let mut first = None;
        let mut second = None;

        if chop.start > start {
            first = Some(Range::new(start, chop.start));
        }

        if chop.end < end {
            second = Some(Range::new(chop.end, end));
        }

        match (first, second) {
            (Some(r1), Some(r2)) => ChopResult::Two(r1, r2),
            (Some(r1), None) => ChopResult::One(r1),
            (None, Some(r2)) => ChopResult::One(r2),
            (None, None) => ChopResult::None,
        }
    }
}

#[derive(Debug)]
pub enum ChopResult {
    None,
    One(Range),
    Two(Range, Range),
}

pub trait BoundExpect {
    fn expect_bound<T>(&self) -> T
    where
        T: TryFrom<Self> + Debug,
        Self: Copy;
}

macro_rules! bound_expect {
    ($for:ty) => {
        impl BoundExpect for $for {
            fn expect_bound<T>(&self) -> T
            where
                T: TryFrom<Self> + Debug,
            {
                match (*self).try_into() {
                    Ok(val) => val,
                    Err(_) => panic!(
                        "bound_expect: {:?} does not fit in {:?}",
                        self,
                        type_name::<T>()
                    ),
                }
            }
        }
    };
}

bound_expect!(i8);
bound_expect!(u8);
bound_expect!(i16);
bound_expect!(u16);
bound_expect!(i32);
bound_expect!(u32);
bound_expect!(i64);
bound_expect!(u64);
bound_expect!(i128);
bound_expect!(u128);
