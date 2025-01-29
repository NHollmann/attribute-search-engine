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
            return QueryParserResult::Freetext(&self.query_str[start_idx..]);
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
    use super::QueryParserResult::*;
    use super::*;

    macro_rules! query_parse_test {
        ($name:ident $query:literal; $($res:expr),* $(,)?) => {
            #[test]
            fn $name() {
                let qp = QueryParser::new($query);
                let result: Vec<QueryParserResult> = qp.collect();
                assert_eq!(result, vec![$($res),*]);
            }
        };
    }

    query_parse_test! {empty "";}
    query_parse_test! {single_char "A"; Freetext("A")}
    query_parse_test! {single_umlaut "Ä"; Freetext("Ä")}
    query_parse_test! {single_emoji "☝🏼"; Freetext("☝🏼")}
    query_parse_test! {single_plus "+"; Freetext("+")}
    query_parse_test! {single_minus "-"; Freetext("-")}
    query_parse_test! {single_colon ":"; Freetext(":")}
    query_parse_test! {single_attribute "+a:b"; Attribute(true, "a", vec!["b"])}
    query_parse_test! {half_attribute "+a"; Freetext("+a")}
    query_parse_test! {plus_colon ":+"; Freetext(":+")}
    query_parse_test! {empty_attribute "+a:"; Attribute(true, "a", vec![])}

    query_parse_test! {
        basic "hello  +zipcode:12345  +pet:Dog  -name:Hans  world";
        Freetext("hello"),
        Attribute(true, "zipcode", vec!["12345"]),
        Attribute(true, "pet", vec!["Dog"]),
        Attribute(false, "name", vec!["Hans"]),
        Freetext("world"),
    }

    query_parse_test! {
        spaces "  \t  hello  +zipcode:12345  \n +pet:Dog  -name:Hans   world    ";
        Freetext("hello"),
        Attribute(true, "zipcode", vec!["12345"]),
        Attribute(true, "pet", vec!["Dog"]),
        Attribute(false, "name", vec!["Hans"]),
        Freetext("world"),
    }

    query_parse_test! {
        comma "+a1:v1 +a2:v1,v2 +a3:v1,v2,v3 -a4:v1,,v2 -a5:v1,v2,";
        Attribute(true, "a1", vec!["v1"]),
        Attribute(true, "a2", vec!["v1", "v2"]),
        Attribute(true, "a3", vec!["v1", "v2", "v3"]),
        Attribute(false, "a4", vec!["v1", "v2"]),
        Attribute(false, "a5", vec!["v1", "v2"]),
    }

    query_parse_test! {
        garbage "\ne376$$bf% sfse-§$\t hello+world ÄÖÜ-+- 😁☝🏼\n\t";
        Freetext("e376$$bf%"),
        Freetext("sfse-§$"),
        Freetext("hello+world"),
        Freetext("ÄÖÜ-+-"),
        Freetext("😁☝🏼"),
    }

    query_parse_test! {
        incomplete " + - +a -b +a-b ";
        Freetext("+"),
        Freetext("-"),
        Freetext("+a"),
        Freetext("-b"),
        Freetext("+a-b"),
    }

    query_parse_test! {
        chained "+a:hello+b:world-foo:+bar,-baz:,buzz";
        Attribute(true, "a", vec!["hello+b:world-foo:+bar", "-baz:", "buzz"]),
    }
}
