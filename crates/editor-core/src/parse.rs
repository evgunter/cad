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

use quantity::{UnitQuantity, unit_by_symbol};

use crate::doc::ParamName;
use crate::expr::{Dimension, DimensionError, Expr};

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
    /// An identifier directly after a number that is not a unit
    /// symbol from the closed table (juxtaposition means nothing else
    /// in this grammar).
    UnknownUnit {
        /// Byte offset of the identifier.
        pos: usize,
        /// The identifier.
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

/// Lex the whole source (byte positions retained per token).
fn lex(src: &str) -> Result<Vec<(usize, Tok)>, ParseError> {
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
            _ => return Err(ParseError::UnexpectedChar { pos, ch }),
        }
    }
    Ok(out)
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
    let toks = lex(src)?;
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
                    let key = ParamName::new(name);
                    match self.params.get(&key) {
                        Some(&dim) => Ok(Expr::param(key, dim)),
                        None => Err(ParseError::UnknownParam { pos, name: key.0 }),
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

    /// `NUMBER [UNIT]` (module docs' literal semantics): suffixed →
    /// continuous literal in canonical units (one f64 multiply); bare
    /// integral → exact Count; bare real → Scalar.
    fn literal(&mut self, pos: usize, text: &str, integral: bool) -> Result<Expr, ParseError> {
        // An identifier DIRECTLY after a number can only be a unit
        // suffix — juxtaposition means nothing else in this grammar.
        if let Some(&(upos, Tok::Ident(ref symbol))) = self.peek() {
            let Some(unit) = unit_by_symbol(symbol) else {
                return Err(ParseError::UnknownUnit {
                    pos: upos,
                    symbol: symbol.clone(),
                });
            };
            self.i += 1;
            let value: f64 = text.parse().map_err(|_| ParseError::MalformedNumber {
                pos,
                text: text.to_string(),
            })?;
            let dim = match unit.quantity() {
                UnitQuantity::Length => Dimension::Length,
                UnitQuantity::Angle => Dimension::Angle,
            };
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
