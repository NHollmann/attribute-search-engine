
trait QueryPredicate {}

struct QueryPredicateEqual {}

impl QueryPredicate for QueryPredicateEqual {}

struct QueryPredicateRange {}

impl QueryPredicate for QueryPredicateRange {}

pub struct Query {
    include_predicates: Vec<Box<dyn QueryPredicate>>,
    exclude_predicates: Vec<Box<dyn QueryPredicate>>,
}
