//! Review probe (exprdepth-rev, not for merging): the merge-base
//! parser, verbatim but for `Expr::neg`'s new `Result`, for a
//! differential fuzz against the rewritten one.
#![allow(clippy::all, clippy::pedantic, clippy::expect_used, dead_code)]
use std::collections::BTreeMap;

use quantity::{UnitDef, unit_by_symbol};

use super::{ParseError, Tok, lex};
use crate::doc::ParamName;
use crate::expr::{Dimension, Expr, UnitSym};

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
            return Ok(Expr::neg(self.unary()?).expect("shallow"));
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
