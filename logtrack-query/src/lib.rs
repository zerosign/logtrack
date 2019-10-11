//
// https://docs.humio.com/language-syntax/
//
// #(tag)=(tag_value) (|:log_op)
// (path)=(path_value) (|:log_op)
// function_call(path_value, ident_like=function_call()) | function_call()
//
use combine::{
    char::{space, string},
    parser::range::take_until,
    Parse, ParseError, Stream,
};

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NegateOp {
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tag<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Filter<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl Tag {
    #[inline]
    pub fn new<'a, S>(name: S, value: S) -> Tag<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Tag {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[inline]
pub fn tag<I>() -> impl Parser<Input = I, Ouput = Tag<'a>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    char('#')
        .with(take_until('='))
        .and(take_until(' '))
        .map(|k, v| Tag::new(k, v))
}

#[inline]
pub fn filter<I>() -> impl Parser<Input = I, Output = Filter<'a>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    take_until('=')
        .and(take_until(' '))
        .map(|k, v| Filter::new(k, v))
}

// create regex like filter

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
