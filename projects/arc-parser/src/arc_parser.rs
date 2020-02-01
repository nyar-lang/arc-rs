pub struct ArcParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    empty_line,
    RestOfLine,
    dict_scope,
    dict_empty,
    dict_head,
    dict_pair,
    list_scope,
    list_empty,
    list_head,
    list_pair,
    dict_literal,
    list_literal,
    Value,
    Null,
    Unit,
    Boolean,
    True,
    False,
    Byte,
    Byte_BIN,
    Byte_OCT,
    Byte_HEX,
    Number,
    SignedNumber,
    Decimal,
    DecimalBad,
    Integer,
    Complex,
    X,
    O,
    B,
    Zero,
    String,
    StringLines,
    StringLiteral,
    StringNormal,
    StringEmpty,
    StringLiteralText,
    StringText,
    StringStart,
    StringEnd,
    Accent,
    Apostrophe,
    Quotation,
    Escape,
    NameSpace,
    Key,
    Dot,
    Dots,
    SYMBOL,
    COMMENT,
    WHITESPACE,
    LineComment,
    MultiLineComment,
    At,
    Sharp,
    Underline,
    Asterisk,
    SEPARATOR,
    Set,
    Vertical,
    Sign,
    Insert,
    Append,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for NoteDownParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic { state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state))))))) } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| self::dict_literal(state)).or_else(|state| state.restore_on_err(|state| self::list_literal(state))).or_else(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::statement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::statement(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::SEPARATOR(state).or_else(|state| state.sequence(|state| self::empty_line(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::empty_line(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::empty_line(state)))))))))).or_else(|state| state.restore_on_err(|state| self::dict_literal(state))).or_else(|state| state.restore_on_err(|state| self::dict_scope(state))).or_else(|state| state.restore_on_err(|state| self::dict_pair(state))).or_else(|state| state.restore_on_err(|state| self::list_scope(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn empty_line(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.sequence(|state| state.optional(|state| self::WHITESPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITESPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::NEWLINE(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RestOfLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::RestOfLine, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dict_scope(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::dict_scope, |state| state.sequence(|state| self::dict_head(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::dict_pair(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::dict_pair(state)))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dict_empty(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::dict_empty, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dict_head(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::dict_head, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::NameSpace(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dict_pair(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::dict_pair, |state| state.sequence(|state| self::NameSpace(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Value(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list_scope(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::list_scope, |state| state.sequence(|state| self::list_head(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::list_pair(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::list_pair(state)))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list_empty(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::list_empty, |state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list_head(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::list_head, |state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| self::NameSpace(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list_pair(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::list_pair, |state| state.restore_on_err(|state| state.sequence(|state| self::Insert(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::dict_pair(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::dict_pair(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::dict_pair(state))))))))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::Append(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Value(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dict_literal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::dict_literal, |state| self::dict_empty(state).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::dict_pair(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::dict_pair(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::dict_pair(state))))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::SEPARATOR(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list_literal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::list_literal, |state| self::list_empty(state).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Value(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Value(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Value(state))))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::SEPARATOR(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]"))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Value(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Value, |state| self::Byte(state).or_else(|state| self::Number(state)).or_else(|state| self::Boolean(state)).or_else(|state| self::Null(state)).or_else(|state| self::Unit(state)).or_else(|state| state.restore_on_err(|state| self::String(state))).or_else(|state| state.restore_on_err(|state| self::dict_literal(state))).or_else(|state| state.restore_on_err(|state| self::list_literal(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Null(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Null, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("null")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Unit(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::Unit, |state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Boolean(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::Boolean, |state| self::True(state).or_else(|state| self::False(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn True(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::True, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("true")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn False(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::False, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("false")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Byte, |state| self::Byte_BIN(state).or_else(|state| self::Byte_OCT(state)).or_else(|state| self::Byte_HEX(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte_BIN(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Byte_BIN, |state| state.sequence(|state| self::Zero(state).and_then(|state| self::B(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_BIN_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_BIN_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte_OCT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Byte_OCT, |state| state.sequence(|state| self::Zero(state).and_then(|state| self::O(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_OCT_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_OCT_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte_HEX(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Byte_HEX, |state| state.sequence(|state| self::Zero(state).and_then(|state| self::X(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_HEX_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Number, |state| self::Complex(state).or_else(|state| self::DecimalBad(state)).or_else(|state| self::SignedNumber(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SignedNumber(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::SignedNumber, |state| state.sequence(|state| state.optional(|state| self::Sign(state)).and_then(|state| self::Decimal(state).or_else(|state| self::Integer(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Decimal, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state)).and_then(|state| self::ASCII_DIGIT(state)).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DecimalBad(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::DecimalBad, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state))).or_else(|state| state.sequence(|state| self::Dot(state).and_then(|state| self::ASCII_DIGIT(state)).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Zero(state).or_else(|state| state.sequence(|state| self::ASCII_NONZERO_DIGIT(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_DIGIT(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Complex(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Complex, |state| state.sequence(|state| self::SignedNumber(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn X(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("x").or_else(|state| state.match_string("X"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn O(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("o").or_else(|state| state.match_string("O"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn B(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("b").or_else(|state| state.match_string("B"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Zero(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("0")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::String, |state| state.sequence(|state| state.optional(|state| self::SYMBOL(state)).and_then(|state| self::StringLines(state).or_else(|state| self::StringNormal(state)).or_else(|state| state.restore_on_err(|state| self::StringLiteral(state))).or_else(|state| self::StringEmpty(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringLines(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringLines, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::NEWLINE(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::WHITESPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITESPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Accent(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::RestOfLine(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.optional(|state| self::NEWLINE(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::WHITESPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITESPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Accent(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::RestOfLine(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.optional(|state| self::NEWLINE(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::WHITESPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITESPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Accent(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::RestOfLine(state)))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringLiteral(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringLiteral, |state| state.sequence(|state| self::StringStart(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringLiteralText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringEnd(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringNormal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringNormal, |state| state.sequence(|state| self::Quotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Quotation(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringEmpty(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringEmpty, |state| state.sequence(|state| self::Quotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Quotation(state))).or_else(|state| state.sequence(|state| self::Apostrophe(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Apostrophe(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringLiteralText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringLiteralText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Apostrophe(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Apostrophe(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Escape(state).or_else(|state| self::Quotation(state)))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| self::Quotation(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Escape(state).or_else(|state| self::Quotation(state)))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| self::Quotation(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringStart(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringStart, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Apostrophe(state).and_then(|state| state.stack_push(|state| state.repeat(|state| self::Apostrophe(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringEnd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringEnd, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::POP(state).and_then(|state| self::Apostrophe(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Accent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Accent, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("`")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Apostrophe(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Apostrophe, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("'")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Quotation(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Quotation, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\"")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Escape(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Escape, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\\")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NameSpace(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NameSpace, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.optional(|state| self::Dots(state)).and_then(|state| self::Key(state)).and_then(|state| state.repeat(|state| state.restore_on_err(|state| state.sequence(|state| self::Dot(state).and_then(|state| self::Key(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Key(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Key, |state| state.restore_on_err(|state| self::String(state)).or_else(|state| self::SYMBOL(state)).or_else(|state| self::Integer(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(".")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dots(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dots, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Dot(state).and_then(|state| state.repeat(|state| self::Dot(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::XID_CONTINUE(state).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::COMMENT, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::MultiLineComment(state).or_else(|state| self::LineComment(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| self::NEWLINE(state).or_else(|state| self::SPACE_SEPARATOR(state)).or_else(|state| state.match_string("\t")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::LineComment, |state| state.sequence(|state| state.match_string("%").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MultiLineComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::MultiLineComment, |state| state.sequence(|state| state.match_string("%%%").and_then(|state| state.repeat(|state| self::MultiLineComment(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("%%%")).and_then(|state| self::ANY(state)))))).and_then(|state| state.match_string("%%%")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn At(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::At, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("@")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sharp(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Sharp, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("#")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Underline(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Underline, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("_")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Asterisk(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Asterisk, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("*")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SEPARATOR, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(",").or_else(|state| state.match_string(";"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Set(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Set, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("=").or_else(|state| state.match_string(":"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Vertical(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Vertical, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("|")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sign(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Sign, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("+").or_else(|state| state.match_string("-"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Insert(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Insert, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("*")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Append(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Append, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">")))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_NONZERO_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_OCT_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'7')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn POP(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_pop()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_HEX_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9').or_else(|state| state.match_range('a'..'f')).or_else(|state| state.match_range('A'..'F'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state| state.match_string("\r\n")).or_else(|state| state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_BIN_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'1')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn SPACE_SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::SPACE_SEPARATOR)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn PEEK(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_peek()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::empty_line => rules::empty_line(state),
            Rule::RestOfLine => rules::RestOfLine(state),
            Rule::dict_scope => rules::dict_scope(state),
            Rule::dict_empty => rules::dict_empty(state),
            Rule::dict_head => rules::dict_head(state),
            Rule::dict_pair => rules::dict_pair(state),
            Rule::list_scope => rules::list_scope(state),
            Rule::list_empty => rules::list_empty(state),
            Rule::list_head => rules::list_head(state),
            Rule::list_pair => rules::list_pair(state),
            Rule::dict_literal => rules::dict_literal(state),
            Rule::list_literal => rules::list_literal(state),
            Rule::Value => rules::Value(state),
            Rule::Null => rules::Null(state),
            Rule::Unit => rules::Unit(state),
            Rule::Boolean => rules::Boolean(state),
            Rule::True => rules::True(state),
            Rule::False => rules::False(state),
            Rule::Byte => rules::Byte(state),
            Rule::Byte_BIN => rules::Byte_BIN(state),
            Rule::Byte_OCT => rules::Byte_OCT(state),
            Rule::Byte_HEX => rules::Byte_HEX(state),
            Rule::Number => rules::Number(state),
            Rule::SignedNumber => rules::SignedNumber(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::DecimalBad => rules::DecimalBad(state),
            Rule::Integer => rules::Integer(state),
            Rule::Complex => rules::Complex(state),
            Rule::X => rules::X(state),
            Rule::O => rules::O(state),
            Rule::B => rules::B(state),
            Rule::Zero => rules::Zero(state),
            Rule::String => rules::String(state),
            Rule::StringLines => rules::StringLines(state),
            Rule::StringLiteral => rules::StringLiteral(state),
            Rule::StringNormal => rules::StringNormal(state),
            Rule::StringEmpty => rules::StringEmpty(state),
            Rule::StringLiteralText => rules::StringLiteralText(state),
            Rule::StringText => rules::StringText(state),
            Rule::StringStart => rules::StringStart(state),
            Rule::StringEnd => rules::StringEnd(state),
            Rule::Accent => rules::Accent(state),
            Rule::Apostrophe => rules::Apostrophe(state),
            Rule::Quotation => rules::Quotation(state),
            Rule::Escape => rules::Escape(state),
            Rule::NameSpace => rules::NameSpace(state),
            Rule::Key => rules::Key(state),
            Rule::Dot => rules::Dot(state),
            Rule::Dots => rules::Dots(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::LineComment => rules::LineComment(state),
            Rule::MultiLineComment => rules::MultiLineComment(state),
            Rule::At => rules::At(state),
            Rule::Sharp => rules::Sharp(state),
            Rule::Underline => rules::Underline(state),
            Rule::Asterisk => rules::Asterisk(state),
            Rule::SEPARATOR => rules::SEPARATOR(state),
            Rule::Set => rules::Set(state),
            Rule::Vertical => rules::Vertical(state),
            Rule::Sign => rules::Sign(state),
            Rule::Insert => rules::Insert(state),
            Rule::Append => rules::Append(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
