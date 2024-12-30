
pub enum Query<'a> {
    ExactString(&'a str, &'a str),

    PrefixString(&'a str, &'a str),

    InRange(&'a str, u64, u64),
    OutRange(&'a str, u64, u64),
    Minimum(&'a str, u64),
    Maximum(&'a str, u64),

    Or(Vec<Query<'a>>),
    And(Vec<Query<'a>>),
    Exclude(Vec<Query<'a>>),
}
