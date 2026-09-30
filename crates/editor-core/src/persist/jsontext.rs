//! **The save body's layout and the canonical bytes, made from compact
//! JSON text without recursing.**
//!
//! A stable name nests one whole name per derivation level, with no
//! bound, so a document's JSON nests as deep as its deepest name. The
//! names write themselves one level at a time inside a JSON door
//! (`names::json_door`); these two passes then give the text the
//! layout each door publishes, each over a heap stack:
//!
//! - [`pretty`]: serde_json's pretty layout (two-space indent, `": "`
//!   between key and value, `[]` and `{}` for empty containers), byte
//!   for byte what `to_string_pretty` writes;
//! - [`canonical`]: every object's keys in sorted order and the root's
//!   `id` removed, byte for byte what serde_json writes for the same
//!   value read into a `serde_json::Value` (whose maps are sorted).

/// The byte index of the `"` closing the string token opening at
/// `open`.
fn string_end(bytes: &[u8], open: usize) -> usize {
    let mut i = open + 1;
    while let Some(&c) = bytes.get(i) {
        match c {
            b'\\' => i += 2,
            b'"' => return i,
            _ => i += 1,
        }
    }
    bytes.len().saturating_sub(1)
}

/// `compact` laid out as serde_json's pretty writer lays out the same
/// value.
pub(super) fn pretty(compact: &str) -> String {
    let bytes = compact.as_bytes();
    let mut out = String::with_capacity(compact.len() * 2);
    let mut depth = 0usize;
    let newline = |out: &mut String, depth: usize| {
        out.push('\n');
        for _ in 0..depth {
            out.push_str("  ");
        }
    };
    let mut i = 0;
    while let Some(&b) = bytes.get(i) {
        match b {
            b'"' => {
                let end = string_end(bytes, i);
                out.push_str(compact.get(i..=end).unwrap_or(""));
                i = end;
            }
            b'[' | b'{' => {
                let close = if b == b'[' { b']' } else { b'}' };
                out.push(char::from(b));
                if bytes.get(i + 1) == Some(&close) {
                    out.push(char::from(close));
                    i += 1;
                } else {
                    depth += 1;
                    newline(&mut out, depth);
                }
            }
            b']' | b'}' => {
                depth = depth.saturating_sub(1);
                newline(&mut out, depth);
                out.push(char::from(b));
            }
            b',' => {
                out.push(',');
                newline(&mut out, depth);
            }
            b':' => out.push_str(": "),
            _ => out.push(char::from(b)),
        }
        i += 1;
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
    let bytes = compact.as_bytes();
    let mut nodes: Vec<Node> = Vec::new();
    // The open containers, and for an object the key its next value
    // belongs under.
    let mut open: Vec<usize> = Vec::new();
    let mut key: Option<((usize, usize), String)> = None;
    let mut i = 0;
    while let Some(&b) = bytes.get(i) {
        let made = match b {
            b'"' => {
                let end = string_end(bytes, i);
                let token = compact.get(i..=end).unwrap_or("");
                let is_key = bytes.get(end + 1) == Some(&b':');
                if is_key {
                    let read = if token.contains('\\') {
                        serde_json::from_str::<String>(token).map_err(|_| KEY)?
                    } else {
                        token.trim_matches('"').to_owned()
                    };
                    key = Some(((i, end + 1), read));
                    i = end + 1;
                    continue;
                }
                let node = nodes.len();
                nodes.push(Node::Scalar(i, end + 1));
                i = end;
                Some(node)
            }
            b'[' | b'{' => {
                let node = nodes.len();
                nodes.push(if b == b'[' {
                    Node::Array(Vec::new())
                } else {
                    Node::Object(Vec::new())
                });
                attach(&mut nodes, &open, &mut key, node);
                open.push(node);
                None
            }
            b']' | b'}' => {
                open.pop();
                None
            }
            b',' | b':' => None,
            _ => {
                let start = i;
                while bytes
                    .get(i + 1)
                    .is_some_and(|c| !matches!(c, b',' | b']' | b'}'))
                {
                    i += 1;
                }
                let node = nodes.len();
                nodes.push(Node::Scalar(start, i + 1));
                Some(node)
            }
        };
        if let Some(node) = made {
            attach(&mut nodes, &open, &mut key, node);
        }
        i += 1;
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
