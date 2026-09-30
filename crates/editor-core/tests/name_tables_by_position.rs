//! **Every corpus document's name tables with every id read as a
//! position**, written to a file for a comparison across two trees.
//!
//! A node id is replaced by the node's position in [`Doc::order`]
//! (`@3`), and a step id by its profile's position, loop and step
//! (`@3/0/2`), so two trees that mint different ids for one recipe
//! print one text wherever the names mean the same thing. Nothing is
//! blanked: which member holds a flush stretch (`FromMember`'s
//! `member`), which face a seam's pair puts first and which way its
//! ranks run all survive as positions, and a stretch that changed
//! hands prints as a different position.
//!
//! Not a gate: it asserts nothing about the names, and writes the dump
//! the comparison reads. To compare two trees, run in each, with the
//! file copied in and listed in `all.rs` where it is absent:
//!
//! ```text
//! CAD_NAME_POSITION_DUMP=/path/<tree>.txt cargo nextest run -p editor-core \
//!     --test all --run-ignored only -E 'test(name_tables_by_position)'
//! diff /path/<a>.txt /path/<b>.txt
//! ```
//!
//! [`Doc::order`]: editor_core::Doc::order

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::fmt::Write as _;

use editor_core::{Node, ProfileDoc};

use crate::corpus;

/// `text` with every `RecipeNodeId(n)` and `StepId(n)` spelled as the
/// position `at` gives it. An id `at` does not hold is left as it was
/// and counted in `unplaced`.
fn at_position(text: &str, at: &BTreeMap<(&str, u64), String>, unplaced: &mut usize) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    'scan: while !rest.is_empty() {
        for tag in ["RecipeNodeId", "StepId"] {
            let open = format!("{tag}(");
            if let Some(tail) = rest.strip_prefix(open.as_str())
                && let Some(close) = tail.find(')')
                && let Ok(bits) = tail[..close].parse::<u64>()
            {
                match at.get(&(tag, bits)) {
                    Some(pos) => out.push_str(pos),
                    None => {
                        *unplaced += 1;
                        out.push_str(&rest[..open.len() + close + 1]);
                    }
                }
                rest = &tail[close + 1..];
                continue 'scan;
            }
        }
        let ch = rest.chars().next().expect("rest is not empty");
        out.push(ch);
        rest = &rest[ch.len_utf8()..];
    }
    out
}

/// Each live node's id and each live profile step's id, as a position.
fn positions(doc: &ProfileDoc) -> BTreeMap<(&'static str, u64), String> {
    let mut at = BTreeMap::new();
    for (pos, id) in doc.order().iter().enumerate() {
        at.insert(("RecipeNodeId", id.0), format!("@{pos}"));
        if let Some(Node::Profile(program)) = doc.node(*id) {
            for (lp, steps) in program.ids.iter().enumerate() {
                for (k, step) in steps.iter().enumerate() {
                    at.insert(("StepId", step.0), format!("@{pos}/{lp}/{k}"));
                }
            }
        }
    }
    at
}

#[test]
#[ignore = "writes the dump a cross-tree comparison reads; see the module doc"]
fn name_tables_by_position() {
    let path = std::env::var("CAD_NAME_POSITION_DUMP")
        .expect("CAD_NAME_POSITION_DUMP names the file to write");
    let mut dump = String::new();
    let mut unplaced = 0usize;
    for d in corpus::documents() {
        let at = positions(&d.doc);
        let ev = corpus::eval::<f64>(&d.doc);
        writeln!(dump, "== {}", d.name).unwrap();
        for (pos, id) in d.doc.order().iter().enumerate() {
            let Some(value) = ev.value(*id) else {
                writeln!(dump, "#{pos} no value").unwrap();
                continue;
            };
            // The table iterates in name order, which reads the ids;
            // sorted after the rewrite, the rows are in one order on
            // both trees.
            let mut rows: Vec<String> = value
                .name_table
                .iter()
                .map(|(n, e)| at_position(&format!("{n:?}={e:?}"), &at, &mut unplaced))
                .collect();
            rows.sort_unstable();
            writeln!(dump, "#{pos} {} rows", rows.len()).unwrap();
            for row in rows {
                writeln!(dump, "  {row}").unwrap();
            }
        }
    }
    writeln!(dump, "unplaced ids: {unplaced}").unwrap();
    std::fs::write(&path, dump).expect("the dump writes");
}
