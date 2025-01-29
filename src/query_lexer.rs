use std::{char, iter::Peekable, str::CharIndices};

#[derive(Debug, PartialEq)]
pub enum QueryToken<'a> {
    Attribute(bool, &'a str, Vec<&'a str>),
    Freetext(&'a str),
}

pub struct QueryLexer<'a> {
    query_str: &'a str,
    char_it: Peekable<CharIndices<'a>>,
}

impl<'a> QueryLexer<'a> {
    pub fn new(query_str: &'a str) -> Self {
        QueryLexer {
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

    fn try_read_attribute(&mut self) -> QueryToken<'a> {
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
            return QueryToken::Freetext(&self.query_str[start_idx..]);
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

        QueryToken::Attribute(first_char == '+', attribute_name, values)
    }

    fn read_freetext(&mut self, start_idx: usize) -> QueryToken<'a> {
        let mut end_idx = start_idx;
        while let Some(&(idx, c)) = self.char_it.peek() {
            end_idx = idx;
            if char::is_whitespace(c) {
                break;
            }
            self.char_it.next();
        }
        if self.char_it.peek().is_none() {
            QueryToken::Freetext(&self.query_str[start_idx..])
        } else {
            QueryToken::Freetext(&self.query_str[start_idx..end_idx])
        }
    }
}

impl<'a> Iterator for QueryLexer<'a> {
    type Item = QueryToken<'a>;

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
    use super::QueryToken::*;
    use super::*;

    macro_rules! query_lexer_test {
        ($name:ident $query:literal; $($res:expr),* $(,)?) => {
            #[test]
            fn $name() {
                let ql = QueryLexer::new($query);
                let result: Vec<QueryToken> = ql.collect();
                assert_eq!(result, vec![$($res),*]);
            }
        };
    }

    query_lexer_test! {empty "";}
    query_lexer_test! {single_char "A"; Freetext("A")}
    query_lexer_test! {single_umlaut "Ä"; Freetext("Ä")}
    query_lexer_test! {single_emoji "☝🏼"; Freetext("☝🏼")}
    query_lexer_test! {single_plus "+"; Freetext("+")}
    query_lexer_test! {single_minus "-"; Freetext("-")}
    query_lexer_test! {single_colon ":"; Freetext(":")}
    query_lexer_test! {single_attribute "+a:b"; Attribute(true, "a", vec!["b"])}
    query_lexer_test! {half_attribute "+a"; Freetext("+a")}
    query_lexer_test! {plus_colon ":+"; Freetext(":+")}
    query_lexer_test! {empty_attribute "+a:"; Attribute(true, "a", vec![])}

    query_lexer_test! {
        basic "hello  +zipcode:12345  +pet:Dog  -name:Hans  world";
        Freetext("hello"),
        Attribute(true, "zipcode", vec!["12345"]),
        Attribute(true, "pet", vec!["Dog"]),
        Attribute(false, "name", vec!["Hans"]),
        Freetext("world"),
    }

    query_lexer_test! {
        spaces "  \t  hello  +zipcode:12345  \n +pet:Dog  -name:Hans   world    ";
        Freetext("hello"),
        Attribute(true, "zipcode", vec!["12345"]),
        Attribute(true, "pet", vec!["Dog"]),
        Attribute(false, "name", vec!["Hans"]),
        Freetext("world"),
    }

    query_lexer_test! {
        comma "+a1:v1 +a2:v1,v2 +a3:v1,v2,v3 -a4:v1,,v2 -a5:v1,v2,";
        Attribute(true, "a1", vec!["v1"]),
        Attribute(true, "a2", vec!["v1", "v2"]),
        Attribute(true, "a3", vec!["v1", "v2", "v3"]),
        Attribute(false, "a4", vec!["v1", "v2"]),
        Attribute(false, "a5", vec!["v1", "v2"]),
    }

    query_lexer_test! {
        garbage "\ne376$$bf% sfse-§$\t hello+world ÄÖÜ-+- 😁☝🏼\n\t";
        Freetext("e376$$bf%"),
        Freetext("sfse-§$"),
        Freetext("hello+world"),
        Freetext("ÄÖÜ-+-"),
        Freetext("😁☝🏼"),
    }

    query_lexer_test! {
        incomplete " + - +a -b +a-b ";
        Freetext("+"),
        Freetext("-"),
        Freetext("+a"),
        Freetext("-b"),
        Freetext("+a-b"),
    }

    query_lexer_test! {
        chained "+a:hello+b:world-foo:+bar,-baz:,buzz";
        Attribute(true, "a", vec!["hello+b:world-foo:+bar", "-baz:", "buzz"]),
    }
}
