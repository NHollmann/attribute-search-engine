
pub trait QueryPredicate {
    fn attribute(&self) -> String;
    fn value(&self) -> String;
}

struct QueryPredicateEqual {
    attr: String,
    value: String,
}

impl QueryPredicate for QueryPredicateEqual {
    fn attribute(&self) -> String {
        self.attr.clone()
    }
    fn value(&self) -> String {
        self.value.clone()
    }
}

struct QueryPredicateRange {}

impl QueryPredicate for QueryPredicateRange {
    fn attribute(&self) -> String {
        String::from("")
    }
    fn value(&self) -> String {
        String::from("")
    }
}

pub struct Query {
    pub include_predicates: Vec<Box<dyn QueryPredicate>>,
    pub exclude_predicates: Vec<Box<dyn QueryPredicate>>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            include_predicates: vec![],
            exclude_predicates: vec![],
        }
    }

    pub fn include_equal(mut self, attribute: &str, value: &str) -> Self {
        self.include_predicates.push(Box::new(QueryPredicateEqual {
            attr: attribute.to_owned(),
            value: value.to_owned(),
        }));
        self
    }

    pub fn exclude_equal(mut self, attribute: &str, value: &str) -> Self {
        self.exclude_predicates.push(Box::new(QueryPredicateEqual {
            attr: attribute.to_owned(),
            value: value.to_owned(),
        }));
        self
    }
}
