mod asn;
mod decode;

use std::{
    io,
    fmt,
    mem,
    ops,
    iter::FromIterator
};
use chrono;

pub use asn::*;
pub use decode::*;

const BYTES: usize = usize::BITS as usize / 8;