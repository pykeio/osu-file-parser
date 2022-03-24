use nom::{
    bytes::complete::{is_not, take_till},
    character::complete::char,
    character::{complete::{multispace0, crlf}, is_newline},
    error::ParseError,
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

pub fn leading_ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(multispace0, inner)
}

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn get_colon_field_value_lines(s: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let field_name = is_not::<_, _, nom::error::Error<_>>(": ");
    let field_separator = ws(char(':'));
    let field_value = is_not(crlf);
    let field = tuple((terminated(field_name, field_separator), field_value));

    many0(field)(s)
}
