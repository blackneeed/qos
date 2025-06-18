use core::option::Option::{Some, None};

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