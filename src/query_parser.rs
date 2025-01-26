use std::{char, iter::Peekable, str::CharIndices};

#[derive(Debug, PartialEq)]
pub enum QueryParserResult<'a> {
    Attribute(bool, &'a str, Vec<&'a str>),
    Freetext(&'a str),
}

pub struct QueryParser<'a> {
    query_str: &'a str,
    char_it: Peekable<CharIndices<'a>>,
}

impl<'a> QueryParser<'a> {
    pub fn new(query_str: &'a str) -> Self {
        QueryParser {
            query_str,
            char_it: query_str.char_indices().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&(_, c)) = self.char_it.peek() {
            if !char::is_whitespace(c) {
                return;
            }
            self.char_it.next();
        }
    }

    fn try_read_attribute(&mut self) -> QueryParserResult<'a> {
        let (start_idx, first_char) = self.char_it.next().unwrap();
        let attr_start_idx = start_idx + 1;
        let mut attr_end_idx = attr_start_idx;

        while let Some(&(idx, c)) = self.char_it.peek() {
            attr_end_idx = idx;
            if c == ':' {
                break;
            }
            if !char::is_alphanumeric(c) {
                break;
            }
            self.char_it.next();
        }

        let mut value_start_idx;
        if let Some(&(column_idx, c)) = self.char_it.peek() {
            if c != ':' {
                return self.read_freetext(start_idx);
            }
            value_start_idx = column_idx + 1;
        } else {
            return QueryParserResult::Freetext(&self.query_str[start_idx..attr_end_idx]);
        }
        self.char_it.next();

        let attribute_name = &self.query_str[attr_start_idx..attr_end_idx];

        let mut values = vec![];
        let mut value_end_idx = 0;
        while let Some(&(idx, c)) = self.char_it.peek() {
            value_end_idx = idx;
            if c == ',' {
                if value_start_idx < value_end_idx {
                    values.push(&self.query_str[value_start_idx..value_end_idx]);
                }
                value_start_idx = idx + 1;
            }
            if char::is_whitespace(c) {
                break;
            }
            self.char_it.next();
        }
        if value_start_idx <= value_end_idx {
            if self.char_it.peek().is_none() {
                values.push(&self.query_str[value_start_idx..]);
            } else {
                values.push(&self.query_str[value_start_idx..value_end_idx]);
            }
        }

        QueryParserResult::Attribute(first_char == '+', attribute_name, values)
    }

    fn read_freetext(&mut self, start_idx: usize) -> QueryParserResult<'a> {
        let mut end_idx = start_idx;
        while let Some(&(idx, c)) = self.char_it.peek() {
            end_idx = idx;
            if char::is_whitespace(c) {
                break;
            }
            self.char_it.next();
        }
        if self.char_it.peek().is_none() {
            QueryParserResult::Freetext(&self.query_str[start_idx..])
        } else {
            QueryParserResult::Freetext(&self.query_str[start_idx..end_idx])
        }
    }
}

impl<'a> Iterator for QueryParser<'a> {
    type Item = QueryParserResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let &(start_idx, first_char) = self.char_it.peek()?;
        if first_char == '+' || first_char == '-' {
            return Some(self.try_read_attribute());
        }
        Some(self.read_freetext(start_idx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_parser_basic() {
        let qp = QueryParser::new("hello  +zipcode:12345  +pet:Dog  -name:Hans  world");
        let result: Vec<QueryParserResult> = qp.collect();
        assert_eq!(
            result,
            vec![
                QueryParserResult::Freetext("hello"),
                QueryParserResult::Attribute(true, "zipcode", vec!["12345"]),
                QueryParserResult::Attribute(true, "pet", vec!["Dog"]),
                QueryParserResult::Attribute(false, "name", vec!["Hans"]),
                QueryParserResult::Freetext("world"),
            ],
        );
    }

    #[test]
    fn query_parser_spaces() {
        let qp =
            QueryParser::new("  \t  hello  +zipcode:12345  \n +pet:Dog  -name:Hans   world    ");
        let result: Vec<QueryParserResult> = qp.collect();
        assert_eq!(
            result,
            vec![
                QueryParserResult::Freetext("hello"),
                QueryParserResult::Attribute(true, "zipcode", vec!["12345"]),
                QueryParserResult::Attribute(true, "pet", vec!["Dog"]),
                QueryParserResult::Attribute(false, "name", vec!["Hans"]),
                QueryParserResult::Freetext("world"),
            ],
        );
    }

    #[test]
    fn query_parser_comma() {
        let qp = QueryParser::new("+a1:v1 +a2:v1,v2 +a3:v1,v2,v3 -a4:v1,,v2 -a5:v1,v2,");
        let result: Vec<QueryParserResult> = qp.collect();
        assert_eq!(
            result,
            vec![
                QueryParserResult::Attribute(true, "a1", vec!["v1"]),
                QueryParserResult::Attribute(true, "a2", vec!["v1", "v2"]),
                QueryParserResult::Attribute(true, "a3", vec!["v1", "v2", "v3"]),
                QueryParserResult::Attribute(false, "a4", vec!["v1", "v2"]),
                QueryParserResult::Attribute(false, "a5", vec!["v1", "v2"]),
            ],
        );
    }

    #[test]
    fn query_parser_garbage() {
        let qp = QueryParser::new("\ne376$$bf% sfse-§$\t hello+world ÄÖÜ-+- 😁☝🏼\n\t");
        let result: Vec<QueryParserResult> = qp.collect();
        assert_eq!(
            result,
            vec![
                QueryParserResult::Freetext("e376$$bf%"),
                QueryParserResult::Freetext("sfse-§$"),
                QueryParserResult::Freetext("hello+world"),
                QueryParserResult::Freetext("ÄÖÜ-+-"),
                QueryParserResult::Freetext("😁☝🏼"),
            ],
        );
    }

    #[test]
    fn query_parser_single_char() {
        let qp = QueryParser::new("A");
        let result: Vec<QueryParserResult> = qp.collect();
        assert_eq!(result, vec![QueryParserResult::Freetext("A")]);
    }

    #[test]
    fn query_parser_single_umlaut() {
        let qp = QueryParser::new("Ä");
        let result: Vec<QueryParserResult> = qp.collect();
        assert_eq!(result, vec![QueryParserResult::Freetext("Ä")]);
    }

    #[test]
    fn query_parser_single_emoji() {
        let qp = QueryParser::new("☝🏼");
        let result: Vec<QueryParserResult> = qp.collect();
        assert_eq!(result, vec![QueryParserResult::Freetext("☝🏼")]);
    }
}
