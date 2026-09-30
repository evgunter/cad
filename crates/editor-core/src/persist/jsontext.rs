//! **The one walk over JSON text**, and the passes built on it.
//!
//! A stable name nests one whole name per derivation level, with no
//! bound, so a document's JSON nests as deep as its deepest name. Every
//! pass that reads JSON as text rather than through serde reads it
//! through [`tokens`], a flat walk that keeps no stack of its own:
//!
//! - the load door's nesting scan and its pruned copy
//!   (`persist::nesting`);
//! - the search for the names inside a name's text (`names::nest`);
//! - [`pretty`]: serde_json's pretty layout (two-space indent, `": "`
//!   between key and value, `[]` and `{}` for empty containers), byte
//!   for byte what `to_string_pretty` writes for every container that
//!   opens within [`BODY_NESTING`] brackets; a container opening deeper
//!   than that is written compact, whole;
//! - [`canonical`]: every object's keys in sorted order and the root's
//!   `id` removed, byte for byte what serde_json writes for the same
//!   value read into a `serde_json::Value` (whose maps are sorted).
//!
//! # Why the layout goes compact past the nesting limit
//!
//! The pretty layout indents a line by its depth, so a name nested `d`
//! brackets deep costs `O(d²)` bytes laid out: 42 MB for a name 1 024
//! `InPart` levels deep. Only a stable name nests past
//! [`BODY_NESTING`], and no body nested past it read back before names
//! were read one level at a time, so a layout that stops indenting
//! there changes no file an earlier build could load, and keeps a
//! file's size linear in its depth.

use super::nesting::BODY_NESTING;

/// What a token of JSON text is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Tok {
    /// `[` or `{`, the byte itself.
    Open(u8),
    /// `]` or `}`, the byte itself.
    Close(u8),
    /// A string that is an object's key: a `:` follows it.
    Key,
    /// Any other string.
    Str,
    /// A number or a literal.
    Scalar,
    /// `,`.
    Comma,
    /// `:`.
    Colon,
}

/// One token of JSON text and the bytes it spans (`start..end`).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Token {
    pub(crate) tok: Tok,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

/// **Every token of `text`, in order**, whitespace skipped. The walk is
/// flat: it knows nothing of nesting, so it costs the stack nothing
/// however deep the text nests. Malformed text still walks (a string
/// left open runs to the end, a stray byte is a scalar); refusing it is
/// the reader's job.
pub(crate) fn tokens(text: &str) -> Tokens<'_> {
    Tokens {
        bytes: text.as_bytes(),
        at: 0,
    }
}

/// The walk [`tokens`] returns.
pub(crate) struct Tokens<'t> {
    bytes: &'t [u8],
    at: usize,
}

fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r')
}

impl Iterator for Tokens<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        while self.bytes.get(self.at).is_some_and(|&b| is_space(b)) {
            self.at += 1;
        }
        let start = self.at;
        let byte = *self.bytes.get(start)?;
        let (tok, end) = match byte {
            b'"' => {
                let mut i = start + 1;
                while let Some(&c) = self.bytes.get(i) {
                    match c {
                        b'\\' => i += 2,
                        b'"' => break,
                        _ => i += 1,
                    }
                }
                let end = (i + 1).min(self.bytes.len());
                let mut after = end;
                while self.bytes.get(after).is_some_and(|&b| is_space(b)) {
                    after += 1;
                }
                let tok = if self.bytes.get(after) == Some(&b':') {
                    Tok::Key
                } else {
                    Tok::Str
                };
                (tok, end)
            }
            b'[' | b'{' => (Tok::Open(byte), start + 1),
            b']' | b'}' => (Tok::Close(byte), start + 1),
            b',' => (Tok::Comma, start + 1),
            b':' => (Tok::Colon, start + 1),
            _ => {
                let mut i = start + 1;
                while self
                    .bytes
                    .get(i)
                    .is_some_and(|&c| !is_space(c) && !matches!(c, b',' | b':' | b']' | b'}'))
                {
                    i += 1;
                }
                (Tok::Scalar, i)
            }
        };
        self.at = end;
        Some(Token { tok, start, end })
    }
}

/// The key a [`Tok::Key`] token spells, its escapes read.
///
/// # Errors
///
/// A key whose escapes do not read as a string.
pub(crate) fn key<'t>(
    text: &'t str,
    token: Token,
) -> Result<std::borrow::Cow<'t, str>, serde_json::Error> {
    let spelled = text.get(token.start..token.end).unwrap_or("");
    if spelled.contains('\\') {
        serde_json::from_str::<String>(spelled).map(std::borrow::Cow::Owned)
    } else {
        Ok(std::borrow::Cow::Borrowed(
            spelled
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .unwrap_or(spelled),
        ))
    }
}

/// Where serde_json's reader stood at `line` and `column` of `text`:
/// the byte offset of that cursor (serde_json counts lines from 1 and
/// columns as the bytes read on the line).
pub(crate) fn cursor(text: &str, line: usize, column: usize) -> usize {
    let start = if line <= 1 {
        0
    } else {
        text.bytes()
            .enumerate()
            .filter(|&(_, b)| b == b'\n')
            .nth(line - 2)
            .map_or(text.len(), |(at, _)| at + 1)
    };
    (start + column).min(text.len())
}

/// [`cursor`]'s inverse: the line and column serde_json's reader
/// reports when it stands at byte `at` of `text`.
pub(crate) fn place(text: &str, at: usize) -> (usize, usize) {
    let before = text.get(..at).unwrap_or(text);
    let line = 1 + before.bytes().filter(|&b| b == b'\n').count();
    let start = before.rfind('\n').map_or(0, |nl| nl + 1);
    (line, at - start)
}

/// `compact` laid out as serde_json's pretty writer lays out the same
/// value, every container that opens past [`BODY_NESTING`] brackets
/// written compact (module docs).
pub(super) fn pretty(compact: &str) -> String {
    let mut out = String::with_capacity(compact.len() * 2);
    let newline = |out: &mut String, depth: usize| {
        out.push('\n');
        for _ in 0..depth {
            out.push_str("  ");
        }
    };
    let mut depth = 0usize;
    // The depth of the container a compact run opened at, while one is
    // open.
    let mut compact_at: Option<usize> = None;
    let mut walk = tokens(compact).peekable();
    while let Some(t) = walk.next() {
        match t.tok {
            Tok::Open(open) => {
                out.push(char::from(open));
                let close = if open == b'[' { b']' } else { b'}' };
                if walk.next_if(|n| n.tok == Tok::Close(close)).is_some() {
                    out.push(char::from(close));
                    continue;
                }
                depth += 1;
                if compact_at.is_none() && depth > BODY_NESTING {
                    compact_at = Some(depth);
                }
                if compact_at.is_none() {
                    newline(&mut out, depth);
                }
            }
            Tok::Close(close) => {
                if compact_at.is_none() {
                    newline(&mut out, depth.saturating_sub(1));
                }
                if compact_at == Some(depth) {
                    compact_at = None;
                }
                depth = depth.saturating_sub(1);
                out.push(char::from(close));
            }
            Tok::Comma => {
                out.push(',');
                if compact_at.is_none() {
                    newline(&mut out, depth);
                }
            }
            Tok::Colon => out.push_str(if compact_at.is_none() { ": " } else { ":" }),
            Tok::Key | Tok::Str | Tok::Scalar => {
                out.push_str(compact.get(t.start..t.end).unwrap_or(""));
            }
        }
    }
    out
}

/// One value of the text [`canonical`] reads.
enum Node {
    /// A string, number or literal: its token's byte range.
    Scalar(usize, usize),
    Array(Vec<usize>),
    /// Each entry's key (its token's byte range, and the key read)
    /// and value.
    Object(Vec<((usize, usize), String, usize)>),
}

/// `compact` with every object's keys sorted and the root object's
/// `id` removed.
///
/// # Errors
///
/// The root is not an object, or holds no `id`: the document did not
/// write the shape this build writes.
pub(super) fn canonical(compact: &str) -> Result<String, &'static str> {
    let mut nodes: Vec<Node> = Vec::new();
    // The open containers, and for an object the key its next value
    // belongs under.
    let mut open: Vec<usize> = Vec::new();
    let mut pending: Option<((usize, usize), String)> = None;
    for t in tokens(compact) {
        let made = match t.tok {
            Tok::Key => {
                let read = key(compact, t).map_err(|_| KEY)?;
                pending = Some(((t.start, t.end), read.into_owned()));
                None
            }
            Tok::Str | Tok::Scalar => {
                nodes.push(Node::Scalar(t.start, t.end));
                Some(nodes.len() - 1)
            }
            Tok::Open(b) => {
                let node = nodes.len();
                nodes.push(if b == b'[' {
                    Node::Array(Vec::new())
                } else {
                    Node::Object(Vec::new())
                });
                attach(&mut nodes, &open, &mut pending, node);
                open.push(node);
                None
            }
            Tok::Close(_) => {
                open.pop();
                None
            }
            Tok::Comma | Tok::Colon => None,
        };
        if let Some(node) = made {
            attach(&mut nodes, &open, &mut pending, node);
        }
    }
    let Some(Node::Object(root)) = nodes.first_mut() else {
        return Err("the document did not serialize as an object");
    };
    let before = root.len();
    root.retain(|(_, k, _)| k != "id");
    if root.len() == before {
        return Err("the document object carries no `id` key");
    }
    for node in &mut nodes {
        if let Node::Object(entries) = node {
            // A key written twice keeps its last value, as a map does.
            entries.reverse();
            entries.sort_by(|a, b| a.1.cmp(&b.1));
            entries.dedup_by(|later, first| later.1 == first.1);
        }
    }
    Ok(write(compact, &nodes))
}

const KEY: &str = "a key did not read back as a string";

/// Puts `node` in the container open innermost, under the pending key
/// if that container is an object.
fn attach(
    nodes: &mut [Node],
    open: &[usize],
    key: &mut Option<((usize, usize), String)>,
    node: usize,
) {
    let Some(parent) = open.last().and_then(|&p| nodes.get_mut(p)) else {
        return;
    };
    match parent {
        Node::Array(items) => items.push(node),
        Node::Object(entries) => {
            if let Some((span, read)) = key.take() {
                entries.push((span, read, node));
            }
        }
        Node::Scalar(..) => {}
    }
}

/// The compact text of `nodes[0]`, from the tokens of `source`.
fn write(source: &str, nodes: &[Node]) -> String {
    let mut out = String::with_capacity(source.len());
    let token = |(start, end): (usize, usize)| source.get(start..end).unwrap_or("");
    // Each open container and how many of its members are written. A
    // value is opened where it is met: a scalar written whole, a
    // container's bracket written and the container pushed.
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut next = Some(0);
    loop {
        if let Some(node) = next.take() {
            match nodes.get(node) {
                Some(Node::Scalar(start, end)) => out.push_str(token((*start, *end))),
                Some(Node::Array(_)) => {
                    out.push('[');
                    stack.push((node, 0));
                }
                Some(Node::Object(_)) => {
                    out.push('{');
                    stack.push((node, 0));
                }
                None => {}
            }
        }
        let Some((node, done)) = stack.last_mut() else {
            break;
        };
        match nodes.get(*node) {
            Some(Node::Array(items)) => match items.get(*done) {
                Some(&item) => {
                    if *done > 0 {
                        out.push(',');
                    }
                    *done += 1;
                    next = Some(item);
                }
                None => {
                    out.push(']');
                    stack.pop();
                }
            },
            Some(Node::Object(entries)) => match entries.get(*done) {
                Some((span, _, value)) => {
                    if *done > 0 {
                        out.push(',');
                    }
                    out.push_str(token(*span));
                    out.push(':');
                    *done += 1;
                    next = Some(*value);
                }
                None => {
                    out.push('}');
                    stack.pop();
                }
            },
            Some(Node::Scalar(..)) | None => {
                stack.pop();
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use std::collections::BTreeMap;

    /// A value with every shape the passes lay out: nesting, empty
    /// containers, fields out of name order, escapes in keys and
    /// strings, numbers and literals.
    #[derive(serde::Serialize)]
    struct Written {
        z: (Vec<f64>, Vec<()>, BTreeMap<String, bool>, Inner),
        id: &'static str,
        a: Option<u8>,
    }

    #[derive(serde::Serialize)]
    struct Inner {
        y: Vec<Vec<Inner>>,
        b: BTreeMap<String, String>,
    }

    /// [`Written`] with its fields, and [`Inner`]'s, in name order and
    /// no `id`: what the canonical bytes of a `Written` must be.
    #[derive(serde::Serialize)]
    struct Sorted {
        a: Option<u8>,
        z: (Vec<f64>, Vec<()>, BTreeMap<String, bool>, InnerSorted),
    }

    #[derive(serde::Serialize)]
    struct InnerSorted {
        b: BTreeMap<String, String>,
        y: Vec<Vec<InnerSorted>>,
    }

    #[test]
    fn both_passes_write_what_serde_json_writes() {
        let keyed = || BTreeMap::from([("b\"q".to_owned(), "x\u{1}y\u{e9}".to_owned())]);
        let written = Written {
            z: (
                vec![1.0, -2.5e-7],
                Vec::new(),
                BTreeMap::new(),
                Inner {
                    y: vec![vec![Inner {
                        y: Vec::new(),
                        b: keyed(),
                    }]],
                    b: BTreeMap::new(),
                },
            ),
            id: "k",
            a: None,
        };
        let sorted = Sorted {
            a: None,
            z: (
                vec![1.0, -2.5e-7],
                Vec::new(),
                BTreeMap::new(),
                InnerSorted {
                    b: BTreeMap::new(),
                    y: vec![vec![InnerSorted {
                        b: keyed(),
                        y: Vec::new(),
                    }]],
                },
            ),
        };
        let compact = serde_json::to_string(&written).unwrap();
        assert_eq!(
            pretty(&compact),
            serde_json::to_string_pretty(&written).unwrap(),
            "pretty"
        );
        assert_eq!(
            canonical(&compact).unwrap(),
            serde_json::to_string(&sorted).unwrap(),
            "canonical"
        );
    }
}
