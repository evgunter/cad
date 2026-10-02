//! **How deep a saved body may nest, and the reader that holds it to
//! that.**
//!
//! A body nests only as deep as the values in it that nest. An
//! expression is bounded where it is minted, by [`MAX_NESTING`], so the
//! deepest body this build saves outside its stable names is known, and
//! [`BODY_NESTING`] is it. A stable name is not bounded: it nests one
//! whole name per derivation level (`names::nest`), and a part chain at
//! its bound already puts a thousand levels into one. So the load door
//! reads a body in two ways at once:
//!
//! - **everything but its names within [`BODY_NESTING`]**: [`read`]
//!   scans the body's bracket nesting first, without recursing, and
//!   refuses a body nested past the limit before any reader descends
//!   into it; within the limit, serde_json reads the body with its own
//!   fixed recursion limit off, since that limit is not derived from
//!   anything here and sits below what the save door writes;
//! - **every name at any depth**: the reader meets each name as a raw
//!   slice of the text and reads it one level at a time, inside a read
//!   door (`names::read_door`), so a name's nesting costs the thread's
//!   stack nothing.
//!
//! # Which brackets the scan leaves to the names
//!
//! A body whose whole nesting is within the limit is read as written.
//! One nested deeper is scanned again with the `path` array of every
//! object that holds one blanked (the text keeps its length and its
//! lines, so every place in it is where it was): a name holds its
//! nesting there and nowhere else. The blanking is a guess, and the
//! reader is what proves it: the door records every name the reader
//! reads, by the offset of its brace, and an object the reader reached
//! at any other type (a `path` array of some other value, or of a
//! user's metadata) is scanned again with its array in place, counted
//! against the limit like everything else, and refused past it. So the
//! scan exempts exactly the places the schema reads a name, whatever
//! the text says, and nothing a hand-edited file can steer: an
//! appearance's user metadata holding a `"kind"`, a pattern whose count
//! sits beside its `"kind"`, or a metadata object shaped like a name is
//! read at its own type and bounded like any other value.
//!
//! An expression reads more narrowly still. Its wire form costs the
//! reader far more stack per level than any other value in a body, and
//! a chain of negations packs one level into each JSON level, so the
//! scan's limit alone would let the reader descend twice the bound. So
//! an expression's children read through [`Child`], which counts the
//! levels the reader is inside and refuses the child that would pass
//! [`MAX_NESTING`] before descending into it, with the constructors'
//! own refusal. The count is a thread-local, the shape serde_json's own
//! recursion limit has; it is restored on every exit, error or not, so
//! nothing one read leaves can reach the next.

use core::cell::Cell;
use std::collections::BTreeSet;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::jsontext::{self, Tok};
use crate::expr::{DimensionError, MAX_NESTING};

/// The most JSON brackets that enclose an expression's root anywhere a
/// save writes one, a measurement's root counting as one of those that
/// enclose the expression its value leaf holds. The deepest is a
/// profile step's target point inside an arc step, in the edit log.
///
/// Not a hand count: `expr_nesting_bound`'s
/// `the_load_doors_limit_is_the_deepest_body_a_save_writes` finds every
/// expression and measurement in the corpus (every node kind and every
/// edit kind), saved both ways, by its wire shape, and fails when the
/// deepest disagrees with [`BODY_NESTING`]; its at-bound row saves an
/// expression at the bound in that position and loads it back.
pub(crate) const ENVELOPE: usize = 14;

/// JSON levels per expression level, at most: a binary operator is a
/// tag object around its operand array (`{"Add": [a, b]}`) and a leaf a
/// tag object around its fields (`{"Literal": {...}}`), two each; a
/// unary operator is the tag object alone (`{"Neg": a}`), one. So an
/// expression nested `n` levels spans at most `2n` brackets from its
/// root's own, and a measurement `2n` from its root's (a primitive leaf
/// is three, and counts one level).
const LEVELS_PER_NESTING: usize = 2;

/// The deepest a body this build saves can nest outside its stable
/// names: an expression or a measurement nested to [`MAX_NESTING`] in
/// the deepest position one sits.
pub(crate) const BODY_NESTING: usize = ENVELOPE + LEVELS_PER_NESTING * MAX_NESTING;

/// Where a body first nests past [`BODY_NESTING`] outside its names:
/// the 1-based line and column of the bracket that does, counted as
/// serde_json counts them (bytes within the line).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TooDeep {
    /// Line within the body.
    pub(crate) line: usize,
    /// Column within the line.
    pub(crate) column: usize,
}

impl core::fmt::Display for TooDeep {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the body nests deeper than {BODY_NESTING} levels outside its stable names, deeper \
             than any saved document does"
        )
    }
}

/// The first bracket that nests `body` past [`BODY_NESTING`], if one
/// does. One flat walk ([`jsontext::tokens`]): brackets inside strings
/// do not count, and malformed JSON is left to the reader.
pub(crate) fn first_too_deep(body: &str) -> Option<TooDeep> {
    let mut depth = 0usize;
    for t in jsontext::tokens(body) {
        match t.tok {
            Tok::Open(_) => {
                depth += 1;
                if depth > BODY_NESTING {
                    let (line, column) = jsontext::place(body, t.end);
                    return Some(TooDeep { line, column });
                }
            }
            Tok::Close(_) => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    None
}

/// A body with the `path` array of every object that holds one
/// blanked, except the objects `kept` names (module docs), and the
/// offset of each object whose array was blanked.
struct Pruned {
    text: String,
    blanked: Vec<usize>,
}

/// `body` pruned: every byte inside a blanked array a space, but for
/// its line breaks, so every place in the text is where it was. An
/// array inside one blanked is blanked with it.
fn prune(body: &str, kept: &BTreeSet<usize>) -> Pruned {
    let mut text = String::with_capacity(body.len());
    let mut blanked = Vec::new();
    let mut copied = 0;
    // The open containers: an object's offset, or `None` for an array.
    let mut open: Vec<Option<usize>> = Vec::new();
    let mut path_next = false;
    let mut walk = jsontext::tokens(body);
    while let Some(t) = walk.next() {
        match t.tok {
            Tok::Key => path_next = jsontext::key(body, t).is_ok_and(|k| k == "path"),
            Tok::Colon => {}
            Tok::Open(b'[')
                if path_next && matches!(open.last(), Some(Some(o)) if !kept.contains(o)) =>
            {
                path_next = false;
                let mut depth = 1usize;
                let close = walk.find(|inner| {
                    match inner.tok {
                        Tok::Open(_) => depth += 1,
                        Tok::Close(_) => depth -= 1,
                        _ => {}
                    }
                    depth == 0
                });
                // An array left open is malformed: the reader refuses it
                // as it stands.
                let (Some(close), Some(Some(object))) = (close, open.last()) else {
                    continue;
                };
                text.push_str(body.get(copied..t.end).unwrap_or(""));
                for c in body.get(t.end..close.start).unwrap_or("").chars() {
                    if matches!(c, '\n' | '\r') {
                        text.push(c);
                    } else {
                        text.extend(core::iter::repeat_n(' ', c.len_utf8()));
                    }
                }
                copied = close.start;
                blanked.push(*object);
            }
            Tok::Open(b) => {
                path_next = false;
                open.push((b == b'{').then_some(t.start));
            }
            Tok::Close(_) => {
                path_next = false;
                open.pop();
            }
            Tok::Str | Tok::Scalar | Tok::Comma => path_next = false,
        }
    }
    text.push_str(body.get(copied..).unwrap_or(""));
    Pruned { text, blanked }
}

/// What [`read`] refuses: a body nested past [`BODY_NESTING`] outside
/// its names, a name's text where it is written, or the reader's own
/// refusal.
#[derive(Debug)]
pub(crate) enum Refused {
    /// The scan's: the body nests too deep to read.
    TooDeep(TooDeep),
    /// A stable name's text, refused where it is written, in the words
    /// the derived form gives (`names::read_door`).
    Name {
        line: usize,
        column: usize,
        category: serde_json::error::Category,
        message: String,
    },
    /// serde_json's.
    Json(serde_json::Error),
}

/// Reads `body` as a `T`: everything but its stable names within
/// [`BODY_NESTING`], and every name at any depth (module docs). Exactly
/// `serde_json::from_str` otherwise, trailing characters included.
///
/// Beside the answer, the typed refusal an impl recorded while reading
/// it (`persist::refusal`): the read that answered is the one the frame
/// is opened around, so nothing a read given up on recorded is
/// reported.
pub(crate) fn read<T: serde::de::DeserializeOwned>(
    body: &str,
) -> (Result<T, Refused>, Option<DimensionError>) {
    let mut kept = BTreeSet::new();
    let mut pruned: Option<Pruned> = None;
    loop {
        let text = pruned.as_ref().map_or(body, |p| p.text.as_str());
        if let Some(at) = first_too_deep(text) {
            if pruned.is_none() {
                pruned = Some(prune(body, &kept));
                continue;
            }
            return (Err(Refused::TooDeep(at)), None);
        }
        let frame = super::refusal::Parse::open();
        let written = pruned.as_ref().map(|_| body);
        let (value, seen) = crate::names::read_door(text, written, || {
            let mut de = serde_json::Deserializer::from_str(text);
            de.disable_recursion_limit();
            let value = T::deserialize(&mut de)?;
            de.end().map(|()| value)
        });
        let refused = frame.finish();
        if let Some(p) = &pruned {
            // Where the reader stopped: past every object it reached.
            let stop = value.as_ref().err().map(|e| {
                seen.fault.as_ref().map_or_else(
                    || jsontext::cursor(text, e.line(), e.column()),
                    |fault| fault.at,
                )
            });
            let misread: Vec<usize> = p
                .blanked
                .iter()
                .copied()
                .filter(|o| !seen.names.contains(o) && stop.is_none_or(|s| *o < s))
                .collect();
            if !misread.is_empty() {
                kept.extend(misread);
                pruned = Some(prune(body, &kept));
                continue;
            }
        }
        let value = value.map_err(|e| match seen.fault {
            Some(fault) => {
                let (line, column) = jsontext::place(body, fault.at);
                Refused::Name {
                    line,
                    column,
                    category: fault.category,
                    message: fault.message,
                }
            }
            None => Refused::Json(e),
        });
        return (value, refused);
    }
}

thread_local! {
    /// How many expression children the reader on this thread is inside.
    static INSIDE: Cell<usize> = const { Cell::new(0) };
}

/// Puts [`INSIDE`] back as it was, however the read it guards exits.
struct Restore(usize);

impl Drop for Restore {
    fn drop(&mut self) {
        INSIDE.set(self.0);
    }
}

/// A child of an expression node on the wire (module docs): written
/// and read as the `Box<T>` it holds, one expression level down.
#[derive(Debug)]
pub(crate) struct Child<T>(pub(crate) Box<T>);

impl<T> Child<T> {
    pub(crate) fn new(value: T) -> Self {
        Self(Box::new(value))
    }
}

impl<T> core::ops::Deref for Child<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Serialize> Serialize for Child<T> {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(ser)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Child<T> {
    /// Refuses the child that would nest past [`MAX_NESTING`], counting
    /// the expression's root as level 1, before reading into it. The
    /// refusal is the constructors' own, recorded for the load door
    /// (`persist::refusal`, which lists this recorder).
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let above = INSIDE.get();
        if above + 2 > MAX_NESTING {
            let refusal = DimensionError::NestedTooDeep { bound: MAX_NESTING };
            super::refusal::record(&refusal);
            return Err(D::Error::custom(format!("expression refused: {refusal}")));
        }
        INSIDE.set(above + 1);
        let _restore = Restore(above);
        T::deserialize(de).map(Self::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_scan_counts_brackets_outside_strings_only() {
        let at_limit = format!("{}{}", "[".repeat(BODY_NESTING), "]".repeat(BODY_NESTING));
        assert_eq!(first_too_deep(&at_limit), None, "the limit itself reads");
        let past = format!("\n  {}", "[".repeat(BODY_NESTING + 1));
        assert_eq!(
            first_too_deep(&past),
            Some(TooDeep {
                line: 2,
                column: 3 + BODY_NESTING
            }),
            "the bracket one past the limit is named"
        );
        let quoted = format!(
            "[\"{}\\\"{}\"]",
            "[".repeat(BODY_NESTING + 1),
            "{".repeat(9)
        );
        assert_eq!(
            first_too_deep(&quoted),
            None,
            "brackets in a string, past an escaped quote too, are text"
        );
    }
}
