/// Query is a recursive datatype that describes a
/// query to a search index or engine.
///
/// The query only takes strings as parameters. It is up to the
/// search index to parse this string into the expected type.
///
/// # Example
/// ```
/// use attribute_search_engine::Query;
///
/// let q = Query::Exclude(
///     Box::new(Query::And(vec![
///         Query::Or(vec![
///             Query::InRange("a".into(), "32".into(), "128".into()),
///             Query::Prefix("b".into(), "hello".into()),
///         ]),
///         Query::Minimum("c".into(), "42".into()),
///     ])),
///     vec![
///         Query::Exact("b".into(), "hello world".into()),
///     ],
/// );
/// ```
#[derive(Debug, PartialEq)]
pub enum Query {
    /// Only matches if the attribute has exactly the value as the query.
    Exact(String, String),

    /// Matches if the attribute starts with the value of the query.
    Prefix(String, String),

    /// Matches if the attribute is in the range of the two query values.
    /// First is minimum, seconds is maximum, both inclusive.
    InRange(String, String, String),

    /// Matches if the attribute is NOT in the range of the two query values.
    /// First is the start of the range, seconds is the end. A value that is
    /// equal to the start or the end is considered in range and will not be
    /// returned.
    OutRange(String, String, String),

    /// Matches if the attribute is at least as high/big as the query value.
    Minimum(String, String),

    /// Matches if the attribute is at most as high/big as the query value.
    Maximum(String, String),

    /// Matches if at least one of the subqueries matches.
    Or(Vec<Query>),

    /// Only matches if all subqueries match.
    And(Vec<Query>),

    /// Removed all matches from the first query that appear in at least
    /// on of the matches of the query vector.
    Exclude(Box<Query>, Vec<Query>),
}

/// Bitmask type for queries that are supported by an index.
pub type SupportedQueries = u8;

/// Signals that an index supports [Exact queries](Query::Exact).
pub const SUPPORTS_EXACT: SupportedQueries = 1 << 0;

/// Signals that an index supports [Prefix queries](Query::Prefix).
pub const SUPPORTS_PREFIX: SupportedQueries = 1 << 1;

/// Signals that an index supports [InRange queries](Query::InRange).
pub const SUPPORTS_INRANGE: SupportedQueries = 1 << 2;

/// Signals that an index supports [OutRange queries](Query::OutRange).
pub const SUPPORTS_OUTRANGE: SupportedQueries = 1 << 3;

/// Signals that an index supports [Minimum queries](Query::Minimum).
pub const SUPPORTS_MINIMUM: SupportedQueries = 1 << 4;

/// Signals that an index supports [Maximum queries](Query::Maximum).
pub const SUPPORTS_MAXIMUM: SupportedQueries = 1 << 5;
