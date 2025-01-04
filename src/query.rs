/// A QueryValue encodes the type and value of a leaf [Query].
#[derive(Debug)]
pub enum QueryValue {
    Str(String),
    Sint64(i64),
    Uint64(u64),
    Sint32(i32),
    Uint32(u32),
    Sint16(i16),
    Uint16(u16),
    Sint8(i8),
    Uint8(u8),
}

/// Query is a recursive datatype that describes a
/// query to a search index or engine.
#[derive(Debug)]
pub enum Query {
    Exact(String, QueryValue),
    Prefix(String, QueryValue),
    InRange(String, QueryValue, QueryValue),
    OutRange(String, QueryValue, QueryValue),
    Minimum(String, QueryValue),
    Maximum(String, QueryValue),

    Or(Vec<Query>),
    And(Vec<Query>),
    Exclude(Box<Query>, Vec<Query>),
}
