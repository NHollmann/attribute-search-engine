/// Query is a recursive datatype that describes a
/// query to a search index or engine.
#[derive(Debug)]
pub enum Query {
    Exact(String, String),
    Prefix(String, String),
    InRange(String, String, String),
    OutRange(String, String, String),
    Minimum(String, String),
    Maximum(String, String),

    Or(Vec<Query>),
    And(Vec<Query>),
    Exclude(Box<Query>, Vec<Query>),
}
