//! The expression TEXT door (LIB-U8a): `&str` → [`Expr`], checking.
//!
//! This is the wire.rs strict-door philosophy at the text door: every
//! reduction goes through [`Expr`]'s fallible smart constructors, so
//! the parser can never mint a tree the constructors refuse — an
//! ill-dimensioned SOURCE STRING is a typed [`ParseError`] carrying
//! the [`DimensionError`] that caused it, never a mis-built tree.
//!
//! GRAMMAR — exactly the AST's span, nothing more (no `^`, no
//! comparisons, no conditionals; F7 totality is the AST's, and the
//! text door adds no vocabulary):
//!
//! ```text
//! expr    := term (('+' | '-') term)*                 left-assoc
//! term    := unary (('*' | '/') unary)*               left-assoc
//! unary   := '-' unary | primary
//! primary := NUMBER [UNIT]                            literals
//!          | IDENT '(' expr (',' expr)* ')'           calls
//!          | IDENT                                    param refs
//!          | '(' expr ')'
//! ```
//!
//! LITERAL SEMANTICS (the ruled fork; quantity's `fmt` module docs
//! carry the other half): a unit-suffixed number (`25 mm`, unit
//! symbols from quantity's closed [`quantity::UNITS`] table) is the
//! decimal's correctly-rounded f64 times the unit factor — ONE f64
//! multiply — landing in canonical kernel units (meters/radians).
//! A UNIT is one or TWO identifier tokens, longest match against the
//! closed table (`pi rad` is a two-word symbol), and nothing but a
//! unit can follow a number here.
//! A BARE INTEGER is a [`Dimension::Count`] literal (exact `i64`); a
//! bare real (`2.0`, `1e3`) is `Scalar`. A unit suffix is the only
//! way a literal acquires a continuous dimension. Count→Scalar
//! promotion is spelled `scalar(n)` — the one call not in the trig/
//! minmax family, chosen as the round-trip fixed point for
//! [`Expr::count_to_scalar`].
//!
//! Functions are `sin cos tan` (Angle→Scalar), `atan2 min max`
//! (binary), `scalar` (Count→Scalar). `-` is both unary and binary
//! with standard precedence; child order is argument order, matching
//! [`Expr::child`]'s indices (persisted `ExprPath`s depend on it —
//! pinned by descend tests).
//!
//! Param refs resolve against the caller's name→dimension table (the
//! doc's params, at the call site's discretion); an unresolved name
//! refuses typed. Names shadow nothing: `sin` followed by `(` is
//! always the call, any other ident position is always a param.

use std::collections::BTreeMap;

use quantity::{UnitDef, unit_by_symbol};

use crate::doc::ParamName;
use crate::expr::{Dimension, DimensionError, Expr, UnitSym};

/// Typed refusal from the text door. Positions are byte offsets into
/// the source string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// A character outside the grammar's alphabet.
    UnexpectedChar {
        /// Byte offset of the character.
        pos: usize,
        /// The character itself.
        ch: char,
    },
    /// The source ended where the grammar required more.
    UnexpectedEnd {
        /// Byte offset of the end of input.
        pos: usize,
        /// What the grammar wanted next.
        expected: &'static str,
    },
    /// A token where the grammar required a different one.
    UnexpectedToken {
        /// Byte offset of the offending token.
        pos: usize,
        /// A rendering of what was found.
        found: String,
        /// What the grammar wanted.
        expected: &'static str,
    },
    /// Input remained after a complete expression.
    TrailingInput {
        /// Byte offset of the first unconsumed token.
        pos: usize,
        /// A rendering of it.
        found: String,
    },
    /// A numeric token `f64`/`i64` refused to read (malformed shape).
    MalformedNumber {
        /// Byte offset of the number.
        pos: usize,
        /// Its text.
        text: String,
    },
    /// A bare integer literal outside `i64` — Count literals are
    /// exact, so an unrepresentable count refuses rather than rounds.
    ///
    /// Corner (inherent to the grammar): `-9223372036854775808`
    /// (i64::MIN) also refuses — the MAGNITUDE lexes as its own token
    /// (9223372036854775808 > i64::MAX) before unary minus applies.
    /// Harmless: no structural count is anywhere near it, and it stays
    /// spellable as an expression if ever needed.
    IntegerOverflow {
        /// Byte offset of the number.
        pos: usize,
        /// Its text.
        text: String,
    },
    /// An identifier directly after a number that begins no unit
    /// symbol of the closed table, at either suffix length
    /// (juxtaposition means nothing else in this grammar).
    UnknownUnit {
        /// Byte offset of the identifier.
        pos: usize,
        /// The identifier — the first token of the refused suffix,
        /// which is the one the reader has to change.
        symbol: String,
    },
    /// A call to a name outside the AST's function vocabulary.
    UnknownFunction {
        /// Byte offset of the identifier.
        pos: usize,
        /// The identifier.
        name: String,
    },
    /// A function called with the wrong number of arguments.
    WrongArity {
        /// Byte offset of the function name.
        pos: usize,
        /// The function.
        name: &'static str,
        /// Its arity.
        expected: usize,
        /// The argument count found.
        found: usize,
    },
    /// An identifier that is not a declared parameter.
    UnknownParam {
        /// Byte offset of the identifier.
        pos: usize,
        /// The identifier.
        name: String,
    },
    /// A reduction the dimension checker refused — the smart
    /// constructors are the only door, so the text door surfaces
    /// their refusal verbatim.
    Dimension {
        /// Byte offset of the token whose reduction was refused (the
        /// operator, function name, or literal).
        pos: usize,
        /// The constructor's refusal.
        error: DimensionError,
    },
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM in the text door's own vocabulary — the byte offset,
// what the grammar wanted, what it found — which for a parser IS the
// recourse: the position plus the expectation is what tells an author
// where to edit. The `Dimension` arm adds position and forwards the
// smart constructor's own refusal rather than re-stating it, because
// the constructors are the only door and their words are the ones that
// hold.
//
// **The quotes are this door's, and they mean "the exact bytes you
// wrote".** Every `{:?}` below renders a `String` lifted verbatim out
// of the author's text — a character, a token, a number, a unit
// symbol, a parameter name — and the delimiter is what says where that
// text began and ended, which matters most when the reason it failed
// is a typo or a stray space. A parameter name is bare everywhere else
// in this crate (`ParamName`'s `Display`), because every other door
// names a parameter the DOCUMENT holds rather than bytes it was handed.
impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnexpectedChar { pos, ch } => write!(
                f,
                "parse: byte {pos}: {ch:?} is outside this grammar's alphabet"
            ),
            Self::UnexpectedEnd { pos, expected } => write!(
                f,
                "parse: byte {pos}: the expression ends where {expected} was required"
            ),
            Self::UnexpectedToken {
                pos,
                found,
                expected,
            } => write!(
                f,
                "parse: byte {pos}: found {found:?} where {expected} was required"
            ),
            Self::TrailingInput { pos, found } => write!(
                f,
                "parse: byte {pos}: {found:?} follows a complete expression — the whole text \
                 has to be one expression"
            ),
            Self::MalformedNumber { pos, text } => write!(
                f,
                "parse: byte {pos}: the number {text:?} is malformed and does not read"
            ),
            Self::IntegerOverflow { pos, text } => write!(
                f,
                "parse: byte {pos}: the count {text:?} does not fit a 64-bit integer — counts \
                 are exact, so an unrepresentable one refuses rather than rounds"
            ),
            Self::UnknownUnit { pos, symbol } => write!(
                f,
                "parse: byte {pos}: {symbol:?} is not a unit symbol, and an identifier directly \
                 after a number means nothing else in this grammar — the unit table is closed"
            ),
            Self::UnknownFunction { pos, name } => write!(
                f,
                "parse: byte {pos}: {name:?} is not a function this expression vocabulary has"
            ),
            Self::WrongArity {
                pos,
                name,
                expected,
                found,
            } => write!(
                f,
                "parse: byte {pos}: {name} takes {expected} argument(s), called with {found}"
            ),
            Self::UnknownParam { pos, name } => write!(
                f,
                "parse: byte {pos}: {name:?} is not a parameter this document declares"
            ),
            Self::Dimension { pos, error } => {
                write!(f, "parse: byte {pos}: {error}")
            }
        }
    }
}

impl core::error::Error for ParseError {}

/// One lexed token.
#[derive(Debug, Clone, PartialEq)]
enum Tok {
    /// A numeric literal, kept as text until the parser knows whether
    /// a unit suffix follows (`integral` = pure digits: no point, no
    /// exponent — the lexer's integer-vs-real decision).
    Number {
        text: String,
        integral: bool,
    },
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Comma,
}

impl Tok {
    /// A short rendering for error text.
    fn describe(&self) -> String {
        match self {
            Self::Number { text, .. } | Self::Ident(text) => text.clone(),
            Self::Plus => "+".to_string(),
            Self::Minus => "-".to_string(),
            Self::Star => "*".to_string(),
            Self::Slash => "/".to_string(),
            Self::LParen => "(".to_string(),
            Self::RParen => ")".to_string(),
            Self::Comma => ",".to_string(),
        }
    }
}

/// Lex the whole source (byte positions retained per token). The one
/// way it refuses is a character outside the alphabet, returned as
/// the offset and the character: [`parse_expr`] words it as
/// [`ParseError::UnexpectedChar`] and [`param_name_fault`] as
/// [`ParamNameReason::OutsideAlphabet`], each in its own vocabulary
/// over the same fact.
fn lex(src: &str) -> Result<Vec<(usize, Tok)>, (usize, char)> {
    let mut out = Vec::new();
    let mut it = src.char_indices().peekable();
    while let Some(&(pos, ch)) = it.peek() {
        match ch {
            c if c.is_whitespace() => {
                it.next();
            }
            '+' | '-' | '*' | '/' | '(' | ')' | ',' => {
                it.next();
                out.push((
                    pos,
                    match ch {
                        '+' => Tok::Plus,
                        '-' => Tok::Minus,
                        '*' => Tok::Star,
                        '/' => Tok::Slash,
                        '(' => Tok::LParen,
                        ')' => Tok::RParen,
                        _ => Tok::Comma,
                    },
                ));
            }
            c if c.is_ascii_digit() => {
                let mut text = String::new();
                let mut integral = true;
                while let Some(&(_, d)) = it.peek() {
                    if d.is_ascii_digit() {
                        text.push(d);
                        it.next();
                    } else {
                        break;
                    }
                }
                if let Some(&(_, '.')) = it.peek() {
                    integral = false;
                    text.push('.');
                    it.next();
                    while let Some(&(_, d)) = it.peek() {
                        if d.is_ascii_digit() {
                            text.push(d);
                            it.next();
                        } else {
                            break;
                        }
                    }
                }
                // An exponent marker only counts when digits (or a
                // signed digit) actually follow — otherwise the `e`
                // starts an identifier token (e.g. a unit suffix).
                let mut ahead = it.clone();
                if let Some((_, 'e' | 'E')) = ahead.next() {
                    let mut exp = String::from("e");
                    if let Some(&(_, s @ ('+' | '-'))) = ahead.peek() {
                        exp.push(s);
                        ahead.next();
                    }
                    if matches!(ahead.peek(), Some(&(_, d)) if d.is_ascii_digit()) {
                        integral = false;
                        text.push_str(&exp);
                        it = ahead;
                        while let Some(&(_, d)) = it.peek() {
                            if d.is_ascii_digit() {
                                text.push(d);
                                it.next();
                            } else {
                                break;
                            }
                        }
                    }
                }
                out.push((pos, Tok::Number { text, integral }));
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut name = String::new();
                while let Some(&(_, d)) = it.peek() {
                    if d.is_alphanumeric() || d == '_' {
                        name.push(d);
                        it.next();
                    } else {
                        break;
                    }
                }
                out.push((pos, Tok::Ident(name)));
            }
            _ => return Err((pos, ch)),
        }
    }
    Ok(out)
}

/// Why a text is not a parameter name, with the text that was offered.
///
/// What [`ParamName::new`] answers. The rule is the lexer's, asked
/// once by `param_name_fault`: a parameter exists to be referenced
/// from an expression, so a name is admissible exactly when the
/// expression parser reads the text back as a reference to that same
/// parameter — one identifier token covering the whole text, and
/// nothing else. Every door that turns text into a name renders this
/// one sentence: the constructor, the load door (through
/// `ParamName`'s `Deserialize`, which is the same constructor) and
/// the bindings above them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamNameFault {
    /// The text offered as a name, verbatim.
    pub offered: String,
    /// What the lexer found in it.
    pub reason: ParamNameReason,
}

impl core::fmt::Display for ParamNameFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Quoted, because this is the one class of sentence that echoes
        // bytes an author typed rather than framing a name the document
        // holds — `ParseError`'s reason, at the constructor.
        write!(f, "parameter name {:?} {}", self.offered, self.reason)
    }
}

impl core::error::Error for ParamNameFault {}

/// The lexer's finding in a text that is not a parameter name: each
/// arm names what was read and where, so a reader knows what to
/// change. Rendered as a predicate phrase after the quoted text
/// ([`ParamNameFault`]'s `Display`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamNameReason {
    /// No token at all: the text is empty or whitespace.
    Blank,
    /// A character the expression alphabet has no token for.
    OutsideAlphabet {
        /// Byte offset of the character.
        pos: usize,
        /// The character itself.
        ch: char,
    },
    /// The text opens with a token that is not an identifier — a
    /// number, an operator, a bracket — so no expression could read it
    /// as a reference.
    NotAnIdentifier {
        /// Byte offset of the token.
        pos: usize,
        /// A rendering of the token.
        found: String,
    },
    /// An identifier, and then more: a second token follows it.
    NotOneToken {
        /// Byte offset of the second token.
        pos: usize,
        /// A rendering of that token.
        found: String,
    },
    /// One identifier, padded with whitespace: the lexer would read the
    /// text back as the trimmed name, which is a different key from
    /// the one offered.
    Padded,
}

impl core::fmt::Display for ParamNameReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Blank => f.write_str("is blank — a parameter name is one identifier"),
            Self::OutsideAlphabet { pos, ch } => write!(
                f,
                "has {ch:?} at byte {pos}, outside the expression alphabet — no expression \
                 could spell this name"
            ),
            Self::NotAnIdentifier { pos, found } => write!(
                f,
                "opens with {found:?} at byte {pos}, which is not an identifier — a parameter \
                 name is one identifier, so an expression can refer to it"
            ),
            Self::NotOneToken { pos, found } => write!(
                f,
                "carries {found:?} at byte {pos} after the identifier — a parameter name is \
                 one identifier, so an expression can refer to it"
            ),
            Self::Padded => f.write_str(
                "is padded with whitespace, which an expression would not read back as part \
                 of the name",
            ),
        }
    }
}

/// **The one admissibility rule for a parameter name**, as the lexer
/// decides it: `None` when the whole text lexes as exactly one
/// identifier token, otherwise the first thing that breaks that shape.
///
/// The parser has no reserved words — `sin` is a call only when `(`
/// follows it, and a bare `sin` is looked up as a parameter — and no
/// constants, so a function word or a unit symbol is admissible here
/// exactly because [`parse_expr`] would read it back as the parameter.
/// `edit_one_predicate::a_name_is_admissible_exactly_when_the_parser_reads_it_back`
/// pins that equivalence rather than restating the grammar here.
pub(crate) fn param_name_fault(text: &str) -> Option<ParamNameReason> {
    let toks = match lex(text) {
        Ok(toks) => toks,
        Err((pos, ch)) => return Some(ParamNameReason::OutsideAlphabet { pos, ch }),
    };
    let mut it = toks.into_iter();
    let Some((pos, first)) = it.next() else {
        return Some(ParamNameReason::Blank);
    };
    let Tok::Ident(ident) = first else {
        return Some(ParamNameReason::NotAnIdentifier {
            pos,
            found: first.describe(),
        });
    };
    if let Some((pos, second)) = it.next() {
        return Some(ParamNameReason::NotOneToken {
            pos,
            found: second.describe(),
        });
    }
    // One identifier token: it covers the whole text exactly when the
    // text has no whitespace around it, since an identifier carries
    // none of its own.
    (ident != text).then_some(ParamNameReason::Padded)
}

/// Parse `src` into a dimension-checked [`Expr`] (module docs: the
/// grammar, the literal semantics, the checking discipline). `params`
/// is the declared parameter table refs resolve against — a document's
/// would be its params' names and dimensions.
///
/// # Errors
///
/// [`ParseError`], including [`ParseError::Dimension`] wrapping the
/// smart constructor's [`DimensionError`] whenever the refusal is
/// dimensional rather than syntactic.
pub fn parse_expr(src: &str, params: &BTreeMap<ParamName, Dimension>) -> Result<Expr, ParseError> {
    let toks = lex(src).map_err(|(pos, ch)| ParseError::UnexpectedChar { pos, ch })?;
    let mut p = Parser {
        toks,
        i: 0,
        end: src.len(),
        params,
    };
    let expr = p.sum()?;
    match p.peek() {
        None => Ok(expr),
        Some((pos, tok)) => Err(ParseError::TrailingInput {
            pos: *pos,
            found: tok.describe(),
        }),
    }
}

struct Parser<'a> {
    toks: Vec<(usize, Tok)>,
    i: usize,
    end: usize,
    params: &'a BTreeMap<ParamName, Dimension>,
}

impl Parser<'_> {
    fn peek(&self) -> Option<&(usize, Tok)> {
        self.toks.get(self.i)
    }

    fn next(&mut self) -> Option<(usize, Tok)> {
        let t = self.toks.get(self.i).cloned();
        if t.is_some() {
            self.i += 1;
        }
        t
    }

    fn expect(&mut self, want: &Tok, expected: &'static str) -> Result<usize, ParseError> {
        match self.next() {
            Some((pos, tok)) if tok == *want => Ok(pos),
            Some((pos, tok)) => Err(ParseError::UnexpectedToken {
                pos,
                found: tok.describe(),
                expected,
            }),
            None => Err(ParseError::UnexpectedEnd {
                pos: self.end,
                expected,
            }),
        }
    }

    /// `expr := term (('+' | '-') term)*` — left-associative, so the
    /// running tree is always the LEFT child (Expr::child index 0).
    fn sum(&mut self) -> Result<Expr, ParseError> {
        let mut acc = self.product()?;
        while let Some(&(pos, ref tok)) = self.peek() {
            let make = match tok {
                Tok::Plus => Expr::add,
                Tok::Minus => Expr::sub,
                _ => break,
            };
            self.i += 1;
            let rhs = self.product()?;
            acc = make(acc, rhs).map_err(|error| ParseError::Dimension { pos, error })?;
        }
        Ok(acc)
    }

    /// `term := unary (('*' | '/') unary)*` — left-associative.
    fn product(&mut self) -> Result<Expr, ParseError> {
        let mut acc = self.unary()?;
        while let Some(&(pos, ref tok)) = self.peek() {
            let make = match tok {
                Tok::Star => Expr::mul,
                Tok::Slash => Expr::div,
                _ => break,
            };
            self.i += 1;
            let rhs = self.unary()?;
            acc = make(acc, rhs).map_err(|error| ParseError::Dimension { pos, error })?;
        }
        Ok(acc)
    }

    /// `unary := '-' unary | primary` (`Expr::neg` is infallible —
    /// negation is total over every dimension, Count included).
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some((_, Tok::Minus)) = self.peek() {
            self.i += 1;
            return Ok(Expr::neg(self.unary()?));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.next() {
            Some((pos, Tok::Number { text, integral })) => self.literal(pos, &text, integral),
            Some((pos, Tok::Ident(name))) => {
                if let Some((_, Tok::LParen)) = self.peek() {
                    self.call(pos, &name)
                } else {
                    // Looked up by the lexed text (`ParamName:
                    // Borrow<str>`), so the parser never mints a name
                    // of its own: the reference it builds is the
                    // table's key, and an identifier the table lacks
                    // is echoed as the bytes read.
                    match self.params.get_key_value(name.as_str()) {
                        Some((key, &dim)) => Ok(Expr::param(key.clone(), dim)),
                        None => Err(ParseError::UnknownParam { pos, name }),
                    }
                }
            }
            Some((_, Tok::LParen)) => {
                let inner = self.sum()?;
                self.expect(&Tok::RParen, "`)`")?;
                Ok(inner)
            }
            Some((pos, tok)) => Err(ParseError::UnexpectedToken {
                pos,
                found: tok.describe(),
                expected: "a literal, parameter, call, or `(`",
            }),
            None => Err(ParseError::UnexpectedEnd {
                pos: self.end,
                expected: "a literal, parameter, call, or `(`",
            }),
        }
    }

    /// The identifier at the cursor, with its byte offset.
    fn peeked_ident(&self) -> Option<(usize, String)> {
        match self.peek() {
            Some((pos, Tok::Ident(name))) => Some((*pos, name.clone())),
            _ => None,
        }
    }

    /// The unit suffix at the cursor: the table row it names and how
    /// many tokens it spans.
    ///
    /// **LONGEST MATCH over consecutive identifier tokens** — two
    /// tokens joined by one space first, then one. The table carries a
    /// two-word symbol (`pi rad`), and a suffix is lexed as
    /// identifiers, so a suffix is a PHRASE and the longest spelling
    /// the closed table has wins.
    ///
    /// There is nothing to disambiguate: juxtaposition after a number
    /// means a unit and nothing else in this grammar, so a second
    /// identifier is never a param reference or a call that the long
    /// match could steal. The fallback exists for the shape `1 rad x`,
    /// where the two-word phrase is not a row and the one-word one is;
    /// `x` then refuses as an unexpected token, which is what it is.
    fn unit_suffix(&self) -> Option<(UnitDef, usize)> {
        let (_, first) = self.peeked_ident()?;
        if let Some((_, Tok::Ident(second))) = self.toks.get(self.i + 1)
            && let Some(unit) = unit_by_symbol(&format!("{first} {second}"))
        {
            return Some((unit, 2));
        }
        unit_by_symbol(&first).map(|unit| (unit, 1))
    }

    /// `NUMBER [UNIT]` (module docs' literal semantics): suffixed →
    /// continuous literal in canonical units (one f64 multiply); bare
    /// integral → exact Count; bare real → Scalar.
    fn literal(&mut self, pos: usize, text: &str, integral: bool) -> Result<Expr, ParseError> {
        // An identifier DIRECTLY after a number can only be a unit
        // suffix — juxtaposition means nothing else in this grammar.
        if let Some((upos, first)) = self.peeked_ident() {
            let Some((unit, consumed)) = self.unit_suffix() else {
                // Nothing matched at either length. The refusal names
                // the FIRST identifier and its own byte offset — the
                // token the reader has to change — rather than a
                // two-word phrase the table was merely asked about.
                return Err(ParseError::UnknownUnit {
                    pos: upos,
                    symbol: first,
                });
            };
            self.i += consumed;
            let value: f64 = text.parse().map_err(|_| ParseError::MalformedNumber {
                pos,
                text: text.to_string(),
            })?;
            // What the suffix MEASURES is one fact, asked once
            // (`UnitSym::measures`) rather than re-laddered here: the
            // literal door re-derives it from the same unit to check
            // the pairing, so a second ladder could only ever agree
            // with the first or make the check refuse a literal the
            // parser had already decided was well-dimensioned.
            //
            // The dimensionless row cannot come back through THIS
            // path — its symbol is the EMPTY string and a suffix here
            // is a parsed IDENTIFIER, never empty — so `Scalar` is an
            // answer only the bare-number path below produces, which
            // reaches it without a lookup.
            let dim = UnitSym::from_def(&unit).measures();
            // The literal REMEMBERS its authored unit (LIB-SWITCH §4g,
            // U8b): canonical value from the one multiply, display
            // unit stored as presentation metadata for the formatter.
            return Expr::literal_with_unit(value * unit.factor(), dim, unit)
                .map_err(|error| ParseError::Dimension { pos, error });
        }
        if integral {
            let value: i64 = text.parse().map_err(|_| ParseError::IntegerOverflow {
                pos,
                text: text.to_string(),
            })?;
            return Ok(Expr::count(value));
        }
        let value: f64 = text.parse().map_err(|_| ParseError::MalformedNumber {
            pos,
            text: text.to_string(),
        })?;
        Expr::literal(value, Dimension::Scalar)
            .map_err(|error| ParseError::Dimension { pos, error })
    }

    /// `IDENT '(' expr (',' expr)* ')'` — the AST's closed function
    /// vocabulary; every application goes through the corresponding
    /// fallible constructor.
    fn call(&mut self, pos: usize, name: &str) -> Result<Expr, ParseError> {
        let (canonical, arity): (&'static str, usize) = match name {
            "sin" => ("sin", 1),
            "cos" => ("cos", 1),
            "tan" => ("tan", 1),
            "scalar" => ("scalar", 1),
            "atan2" => ("atan2", 2),
            "min" => ("min", 2),
            "max" => ("max", 2),
            _ => {
                return Err(ParseError::UnknownFunction {
                    pos,
                    name: name.to_string(),
                });
            }
        };
        self.expect(&Tok::LParen, "`(`")?;
        let mut args = vec![self.sum()?];
        while let Some((_, Tok::Comma)) = self.peek() {
            self.i += 1;
            args.push(self.sum()?);
        }
        self.expect(&Tok::RParen, "`)` or `,`")?;
        if args.len() != arity {
            return Err(ParseError::WrongArity {
                pos,
                name: canonical,
                expected: arity,
                found: args.len(),
            });
        }
        let mut it = args.into_iter();
        let (Some(a), b) = (it.next(), it.next()) else {
            // args starts non-empty and the arity check bounds it;
            // typed rather than trusted (no panic paths).
            return Err(ParseError::WrongArity {
                pos,
                name: canonical,
                expected: arity,
                found: 0,
            });
        };
        let built = match (canonical, b) {
            ("sin", None) => Expr::sin(a),
            ("cos", None) => Expr::cos(a),
            ("tan", None) => Expr::tan(a),
            ("scalar", None) => Expr::count_to_scalar(a),
            ("atan2", Some(b)) => Expr::atan2(a, b),
            ("min", Some(b)) => Expr::min(a, b),
            ("max", Some(b)) => Expr::max(a, b),
            // Arity was just checked; unreachable, kept typed.
            (_, _) => {
                return Err(ParseError::WrongArity {
                    pos,
                    name: canonical,
                    expected: arity,
                    found: usize::MAX,
                });
            }
        };
        built.map_err(|error| ParseError::Dimension { pos, error })
    }
}
