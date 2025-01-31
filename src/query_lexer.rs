use std::{char, iter::Peekable, str::CharIndices};

/// A single token in a query string.
/// It only saves slices into the source query.
#[derive(Debug, PartialEq)]
pub enum QueryToken<'a> {
    /// A full attribute in the query string.
    ///
    /// The boolean indicates if the attribute is inclusive or exclusive.
    /// The first string slice is the name of the index, the vector of string
    /// slices saves the attribute values that are queried.
    Attribute(bool, &'a str, Vec<&'a str>),

    /// A non-relevant non-whitespace part of the query string.
    Freetext(&'a str),
}

/// QueryLexer is an iterator that takes a string slice and returns
/// [QueryTokens](QueryToken) for each relevant section in the input slice.
pub struct QueryLexer<'a> {
    query_str: &'a str,
    char_it: Peekable<CharIndices<'a>>,
}

impl<'a> QueryLexer<'a> {
    /// Creates a new QueryLexer object.
    pub fn new(query_str: &'a str) -> Self {
        QueryLexer {
            query_str,
            char_it: query_str.char_indices().peekable(),
        }
    }

    /// Return the next token found or None if the query_str is
    /// exhausted.
    fn next_token(&mut self) -> Option<QueryToken<'a>> {
        self.skip_whitespace();

        let &(start_idx, first_char) = self.char_it.peek()?;
        if first_char == '+' || first_char == '-' {
            return Some(self.read_attribute());
        }
        Some(self.read_freetext(start_idx))
    }

    /// Skip whitespace in input.
    fn skip_whitespace(&mut self) {
        while let Some(&(_, c)) = self.char_it.peek() {
            if !char::is_whitespace(c) {
                return;
            }
            self.char_it.next();
        }
    }

    /// Read until the first whitespace character or the end of the
    /// string slice and return a [Freetext Token](QueryToken::Freetext).
    fn read_freetext(&mut self, start_idx: usize) -> QueryToken<'a> {
        while let Some(&(idx, c)) = self.char_it.peek() {
            if char::is_whitespace(c) {
                return QueryToken::Freetext(&self.query_str[start_idx..idx]);
            }
            self.char_it.next();
        }
        QueryToken::Freetext(&self.query_str[start_idx..])
    }

    /// Read a full attribute including index name and a vector of values.
    /// On success an [Attribute Token](QueryToken::Attribute) is returned.
    /// If at some point the input is malformed, a [Freetext Token](QueryToken::Freetext)
    /// is returned instead.
    fn read_attribute(&mut self) -> QueryToken<'a> {
        let (start_idx, first_char) = self.char_it.next().unwrap();

        let (attribute_index, attribute_ok) = self.read_attribute_index(start_idx + 1);
        if !attribute_ok || attribute_index.is_empty() {
            return self.read_freetext(start_idx);
        }
        let (colon_idx, c) = self
            .char_it
            .next()
            .expect("if attribute_ok is true there must be a next char");
        assert_eq!(
            c, ':',
            "if attribute_ok is true, the next char should be a colon"
        );

        let attribute_values = self.read_attribute_values(colon_idx + 1);
        QueryToken::Attribute(first_char == '+', attribute_index, attribute_values)
    }

    /// Read the name of an attribute index. Stop if a colon, a
    /// unexpected character or the end of the string is found.
    /// The second value of the result tuple indicates if a colon
    /// was found at the end.
    fn read_attribute_index(&mut self, start_idx: usize) -> (&'a str, bool) {
        while let Some(&(idx, c)) = self.char_it.peek() {
            if c == ':' || !char::is_alphanumeric(c) {
                return (&self.query_str[start_idx..idx], c == ':');
            }
            self.char_it.next();
        }
        // If we are at the end of the query string, this can't be a valid
        // attribute.
        ("", false)
    }

    /// Read a vector of comma seperated attributes from the query string.
    fn read_attribute_values(&mut self, mut value_start_idx: usize) -> Vec<&'a str> {
        let mut values = vec![];

        while let Some(&(idx, c)) = self.char_it.peek() {
            if c == ',' || char::is_whitespace(c) {
                // We only push non-empty values to our result vector.
                if value_start_idx < idx {
                    values.push(&self.query_str[value_start_idx..idx]);
                }

                // If we found a whitespace, we exit directly.
                if char::is_whitespace(c) {
                    return values;
                }

                value_start_idx = idx + 1;
            }
            self.char_it.next();
        }

        // This case may happen if the last value is at the end of the query string.
        // If it is not empty, we want to append it to the result vector.
        let last_value = &self.query_str[value_start_idx..];
        if !last_value.is_empty() {
            values.push(last_value);
        }

        values
    }
}

impl<'a> Iterator for QueryLexer<'a> {
    type Item = QueryToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
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
    query_lexer_test! {plus_colon "+:"; Freetext("+:")}
    query_lexer_test! {colon_plus ":+"; Freetext(":+")}
    query_lexer_test! {empty_attribute "+a:"; Attribute(true, "a", vec![])}
    query_lexer_test! {empty_attribute_space "+a: "; Attribute(true, "a", vec![])}

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
        comma "+a1:v1 +a2:v1,v2 +a3:v1,v2,v3 -a4:v1,,v2 -a5:v1,v2, +a6:,,,";
        Attribute(true, "a1", vec!["v1"]),
        Attribute(true, "a2", vec!["v1", "v2"]),
        Attribute(true, "a3", vec!["v1", "v2", "v3"]),
        Attribute(false, "a4", vec!["v1", "v2"]),
        Attribute(false, "a5", vec!["v1", "v2"]),
        Attribute(true, "a6", vec![]),
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
