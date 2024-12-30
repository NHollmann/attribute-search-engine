use crate::error::*;
use regex::Regex;

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

impl<'a> Query<'a> {
    pub fn from_str(query_str: &'a str) -> Result<Self> {
        // TODO: Support numbers, comma sperators (OR) and minus symbols (RANGES)
        let re = Regex::new(r"([\+-])(\w):(\w)").expect("the regex to compile");
        let mut results = vec![];
        for (_, [modifiery, attribute, value]) in re.captures_iter(query_str).map(|c| c.extract()) {
            results.push((
                modifiery,
                attribute,
                value,
            ));
        }
        // TODO: Transform captures to a query
        Ok(Query::ExactString(query_str, QueryValue::Str(query_str)))
    }
}
