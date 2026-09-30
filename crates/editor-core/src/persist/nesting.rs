//! **How deep a saved body may nest, and the reader that holds it to
//! that.**
//!
//! A body nests only as deep as the values in it that nest, and each of
//! those is bounded where it is minted: an expression by
//! [`MAX_NESTING`]. So the deepest body this build saves is known, and
//! [`BODY_NESTING`] is it. The load door reads exactly that far:
//! [`read`] scans the body's bracket nesting first, without recursing,
//! and refuses a body nested past the limit before any reader descends
//! into it; within the limit, serde_json reads the body with its own
//! fixed recursion limit off, since that limit is not derived from
//! anything here and sits below what the save door writes.
//!
//! The scan is what keeps the reader's recursion on a small stack: no
//! body it admits nests past [`BODY_NESTING`], whatever types it holds.
//! A value that nests deeper than an expression can adds its own term
//! to [`BODY_NESTING`] beside the expression's, and the scan reads the
//! larger.
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

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
pub(crate) const ENVELOPE: usize = 15;

/// JSON levels per expression level, at most: a binary operator is a
/// tag object around its operand array (`{"Add": [a, b]}`) and a leaf a
/// tag object around its fields (`{"Literal": {...}}`), two each; a
/// unary operator is the tag object alone (`{"Neg": a}`), one. So an
/// expression nested `n` levels spans at most `2n` brackets from its
/// root's own, and a measurement `2n` from its root's (a primitive leaf
/// is three, and counts one level).
const LEVELS_PER_NESTING: usize = 2;

/// The deepest a body this build saves can nest: an expression or a
/// measurement nested to [`MAX_NESTING`] in the deepest position one
/// sits.
pub(crate) const BODY_NESTING: usize = ENVELOPE + LEVELS_PER_NESTING * MAX_NESTING;

/// Where a body first nests past [`BODY_NESTING`]: the 1-based line and
/// column of the bracket that does, counted as serde_json counts them
/// (bytes within the line).
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
            "the body nests deeper than {BODY_NESTING} levels, the deepest a saved document \
             reaches"
        )
    }
}

/// The first bracket that nests `body` past [`BODY_NESTING`], if one
/// does. A single pass with a counter: brackets inside strings (escapes
/// included) do not count, and malformed JSON is left to the reader.
pub(crate) fn first_too_deep(body: &str) -> Option<TooDeep> {
    let (mut depth, mut line, mut line_start) = (0usize, 1usize, 0usize);
    let (mut in_string, mut escaped) = (false, false);
    for (at, byte) in body.bytes().enumerate() {
        if in_string {
            match byte {
                _ if escaped => escaped = false,
                b'\\' => escaped = true,
                b'"' => in_string = false,
                _ => {}
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'\n' => {
                line += 1;
                line_start = at + 1;
            }
            b'[' | b'{' => {
                depth += 1;
                if depth > BODY_NESTING {
                    return Some(TooDeep {
                        line,
                        column: at - line_start + 1,
                    });
                }
            }
            b']' | b'}' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    None
}

/// What [`read`] refuses: a body nested past [`BODY_NESTING`], or the
/// reader's own refusal.
#[derive(Debug)]
pub(crate) enum Refused {
    /// The scan's: the body nests too deep to read.
    TooDeep(TooDeep),
    /// serde_json's.
    Json(serde_json::Error),
}

/// Reads `body` as a `T` when it nests no deeper than [`BODY_NESTING`]
/// (module docs). Exactly `serde_json::from_str` otherwise, trailing
/// characters included.
pub(crate) fn read<T: serde::de::DeserializeOwned>(body: &str) -> Result<T, Refused> {
    if let Some(at) = first_too_deep(body) {
        return Err(Refused::TooDeep(at));
    }
    let mut de = serde_json::Deserializer::from_str(body);
    de.disable_recursion_limit();
    let value = T::deserialize(&mut de).map_err(Refused::Json)?;
    de.end().map_err(Refused::Json)?;
    Ok(value)
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
