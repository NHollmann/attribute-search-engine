
/// Query value sum type
pub enum QueryValue<'a> {
    Str(&'a str),
    Sint64(i64),
    Uint64(u64),
    Sint32(i32),
    Uint32(u32),
    Sint16(i16),
    Uint16(u16),
    Sint8(i8),
    Uint8(u8),
}

/// Query sum type
pub enum Query<'a> {
    ExactString(&'a str, QueryValue<'a>),

    PrefixString(&'a str, QueryValue<'a>),

    InRange(&'a str, QueryValue<'a>, QueryValue<'a>),
    OutRange(&'a str, QueryValue<'a>, QueryValue<'a>),
    Minimum(&'a str, QueryValue<'a>),
    Maximum(&'a str, QueryValue<'a>),

    Or(Vec<Query<'a>>),
    And(Vec<Query<'a>>),
    Exclude(Vec<Query<'a>>),
}
