//! Review probes (lane `exprdepth-rev`, PR 3510), not for merging.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::{
    DimensionError, Expr, MeasureExpr, Node, ParseError, ProfileDoc, content_pin,
    parse_expr,
};
use fixture::{Recorder, len, xy_frame};
use geom_core::Tol;

const BOUND: usize = 128;
const WASM_STACK: usize = 1 << 20;

fn on_stack<R: Send + 'static>(size: usize, f: impl FnOnce() -> R + Send + 'static) -> R {
    std::thread::Builder::new()
        .stack_size(size)
        .spawn(f)
        .expect("the thread starts")
        .join()
        .expect("the subject returns")
}

fn deep_length(metres: f64, levels: usize) -> Expr {
    (1..levels).fold(len(metres), |e, _| Expr::add(e, len(0.0)).unwrap())
}

/// A measurement nesting `total` levels: `measure_levels` measurement
/// levels of negation over a value leaf whose expression nests the rest.
fn mixed(total: usize, measure_levels: usize) -> Result<MeasureExpr, DimensionError> {
    let leaf = MeasureExpr::value(deep_length(0.5, total - measure_levels + 1));
    (1..measure_levels).try_fold(leaf, |m, _| MeasureExpr::neg(m))
}

fn doc_with(measure: MeasureExpr, distance: Expr) -> Recorder {
    let mut r = Recorder::new();
    let plane = r.insert(xy_frame());
    let profile = r.insert(Node::Profile(fixture::desc(
        plane,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    r.insert(Node::Extrude { profile, distance });
    r.insert(Node::Measure {
        expr: measure,
        refs: vec![],
    });
    r
}

fn round_trips(r: &Recorder) -> Result<(), String> {
    let pin = content_pin(&r.doc, Tol::witness()).map_err(|e| format!("pin: {e}"))?;
    let empty = ProfileDoc::empty_derived("mod", Tol::witness());
    for (label, snapshot, edits) in [("snapshot", &r.doc, &[][..]), ("log", &empty, &r.edits[..])] {
        let text = editor_core::persist::save(snapshot, edits, Tol::witness())
            .map_err(|e| format!("{label} save: {e}"))?;
        let loaded = editor_core::persist::load(&text, Tol::witness())
            .map_err(|e| format!("{label} load: {e}"))?;
        let back = content_pin(&loaded.doc, Tol::witness()).map_err(|e| format!("{e}"))?;
        if back != pin {
            return Err(format!("{label}: pin moved"));
        }
    }
    Ok(())
}

/// Claim 3: mixed Expr/MeasureExpr nesting at the bound round-trips on
/// 1 MiB at every split; one past refuses at the constructor.
#[test]
fn review_mixed_nesting_round_trips_at_every_split() {
    on_stack(WASM_STACK, || {
        for m in [1, 2, 3, 64, 100, 127, 128] {
            let at = mixed(BOUND, m).unwrap_or_else(|e| panic!("split {m}: {e}"));
            let r = doc_with(at, deep_length(0.5, BOUND));
            round_trips(&r).unwrap_or_else(|e| panic!("split {m}: {e}"));
            if m < BOUND {
                assert!(
                    matches!(mixed(BOUND + 1, m + 1), Err(DimensionError::NestedTooDeep { .. })),
                    "split {m}: one past refuses"
                );
            }
        }
    });
}

fn save_of(r: &Recorder) -> String {
    editor_core::persist::save(&r.doc, &[], Tol::witness()).unwrap()
}

fn wrap_distance(text: &str, prefix: &str, levels: usize) -> String {
    let key = "\"distance\": ";
    let at = text.find(key).unwrap() + key.len();
    let mut depth = 0usize;
    let mut end = at;
    for (i, c) in text[at..].char_indices() {
        match c {
            '{' | '[' => depth += 1,
            '}' | ']' => {
                depth -= 1;
                if depth == 0 {
                    end = at + i + 1;
                    break;
                }
            }
            _ => {}
        }
    }
    format!(
        "{}{}{}{}{}",
        &text[..at],
        prefix.repeat(levels),
        &text[at..end],
        "}".repeat(levels),
        &text[end..]
    )
}

/// Claim 4: the thread-local counter after refusals on the same thread,
/// including a refusal raised deep inside `Child` by something other
/// than `Child` itself (an unknown unit, an unknown variant), then an
/// at-bound document loads on the same thread.
#[test]
fn review_counter_resets_after_every_refusal_on_the_same_thread() {
    on_stack(WASM_STACK, || {
        let good = doc_with(mixed(BOUND, 64).unwrap(), deep_length(0.5, BOUND));
        let good_text = save_of(&good);
        let shallow = doc_with(mixed(2, 1).unwrap(), len(0.5));
        let shallow_text = save_of(&shallow);
        let mut bad = Vec::new();
        // Child's own refusal, at 129 and at 200.
        bad.push(("child 129", wrap_distance(&shallow_text, "{\"Neg\": ", BOUND)));
        bad.push(("child 200", wrap_distance(&shallow_text, "{\"Neg\": ", 200)));
        // A leaf refused deep inside Child: an unknown unit symbol 100 levels down.
        let unit = wrap_distance(&shallow_text, "{\"Neg\": ", 100).replacen("\"unit\": \"", "\"unit\": \"zz", 1);
        bad.push(("unit 100 deep", unit));
        // An unknown variant 100 levels down.
        let variant = wrap_distance(&shallow_text, "{\"Neg\": ", 99).replacen(
            "\"distance\": {\"Neg\": ",
            "\"distance\": {\"Nope\": ",
            1,
        );
        bad.push(("variant", variant));
        // Truncated mid-expression, 120 levels down.
        let deep = wrap_distance(&shallow_text, "{\"Neg\": ", 120);
        let cut = deep.find("\"unit\"").unwrap();
        bad.push(("truncated", deep[..cut].to_string()));
        for (label, text) in &bad {
            let refused = editor_core::persist::load(text, Tol::witness());
            assert!(refused.is_err(), "{label} refuses");
            eprintln!("REVIEW {label}: {}", refused.err().unwrap());
            // The at-bound document loads right after, on the same thread.
            editor_core::persist::load(&good_text, Tol::witness())
                .unwrap_or_else(|e| panic!("after {label}: {e}"));
            // And a 127-deep negation wrap still loads (no leaked count).
            let near = wrap_distance(&shallow_text, "{\"Neg\": ", BOUND - 2);
            editor_core::persist::load(&near, Tol::witness())
                .unwrap_or_else(|e| panic!("after {label}, the near-bound wrap: {e}"));
        }
    });
}

/// Claim 4: concurrent loads on two 1 MiB threads, interleaving
/// refusals and at-bound loads.
#[test]
fn review_concurrent_loads_keep_their_own_count() {
    let good = doc_with(mixed(BOUND, 64).unwrap(), deep_length(0.5, BOUND));
    let good_text = std::sync::Arc::new(save_of(&good));
    let shallow_text = save_of(&doc_with(mixed(2, 1).unwrap(), len(0.5)));
    let bad = std::sync::Arc::new(wrap_distance(&shallow_text, "{\"Neg\": ", 200));
    let near = std::sync::Arc::new(wrap_distance(&shallow_text, "{\"Neg\": ", BOUND - 2));
    let handles: Vec<_> = (0..4)
        .map(|t| {
            let (good, bad, near) = (good_text.clone(), bad.clone(), near.clone());
            std::thread::Builder::new()
                .stack_size(WASM_STACK)
                .spawn(move || {
                    for i in 0..20 {
                        if (i + t) % 2 == 0 {
                            assert!(editor_core::persist::load(&bad, Tol::witness()).is_err());
                        }
                        editor_core::persist::load(&good, Tol::witness()).unwrap();
                        editor_core::persist::load(&near, Tol::witness()).unwrap();
                    }
                })
                .unwrap()
        })
        .collect();
    for h in handles {
        h.join().expect("a loader thread survives");
    }
}

/// Claim 4: a crafted file nested past the scan's limit in a
/// non-expression position, and one at the limit, on 1 MiB.
#[test]
fn review_non_expression_nesting_at_and_past_the_scan() {
    on_stack(WASM_STACK, || {
        let text = save_of(&doc_with(mixed(2, 1).unwrap(), len(0.5)));
        // A list bomb in place of the measurement's refs array.
        for (label, n) in [("at 289", 289 - 3), ("past", 400), ("far past", 100_000)] {
            let bomb = format!("{}{}", "[".repeat(n), "]".repeat(n));
            let crafted = text.replacen("\"refs\": []", &format!("\"refs\": {bomb}"), 1);
            assert_ne!(crafted, text);
            let out = editor_core::persist::load(&crafted, Tol::witness());
            eprintln!("REVIEW bomb {label}: {:?}", out.as_ref().err().map(ToString::to_string));
            assert!(out.is_err());
        }
    });
}

/// Claim 2: the text door at the bound with every construct stacked:
/// brackets at their limit around a sum at its limit; and 129 brackets
/// around a depth-1 literal (what the bracket bound refuses).
#[test]
fn review_text_door_bracket_bound_is_independent_of_the_value() {
    on_stack(WASM_STACK, || {
        let sum = vec!["1"; BOUND].join("+");
        let wrapped = format!("{}{sum}{}", "(".repeat(BOUND), ")".repeat(BOUND));
        let e = parse_expr(&wrapped, &BTreeMap::new());
        assert!(e.is_ok(), "{e:?}");
        let lone = format!("{}1{}", "(".repeat(BOUND + 1), ")".repeat(BOUND + 1));
        match parse_expr(&lone, &BTreeMap::new()) {
            Err(ParseError::Dimension { error, .. }) => {
                eprintln!("REVIEW a depth-1 literal in 129 brackets refuses: {error}");
            }
            other => panic!("{other:?}"),
        }
    });
}

/// Claim 2/3: every tree the constructors admit at the bound reads back
/// through `unparse` → `parse_expr` (the text door's bracket bound must
/// not refuse a rendering of a legal tree).
#[test]
fn review_unparse_of_any_tree_at_the_bound_reads_back() {
    use editor_core::{Dimension, unparse};
    on_stack(WASM_STACK, || {
        let mut seed = 0x2545_F491_4F6C_DD1Du64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let lit = |v: f64| Expr::literal(v, Dimension::Scalar).unwrap();
        for trial in 0..2000 {
            let mut e = lit(1.5);
            for _ in 2..BOUND {
                let other = if next() % 3 == 0 { Expr::neg(lit(0.25)).unwrap() } else { lit(0.25) };
                let right = next() % 2 == 0;
                let (a, b) = if right { (other, e) } else { (e, other) };
                e = match next() % 7 {
                    0 => Expr::add(a, b),
                    1 => Expr::sub(a, b),
                    2 => Expr::mul(a, b),
                    3 => Expr::div(a, b),
                    4 => Expr::min(a, b),
                    5 => Expr::max(a, b),
                    _ => Expr::neg(if right { b } else { a }),
                }
                .unwrap_or_else(|err| panic!("build: {err}"));
            }
            let text = unparse(&e);
            let back = parse_expr(&text, &BTreeMap::new())
                .unwrap_or_else(|err| panic!("trial {trial}: {err}\n{text}"));
            assert!(back.bit_eq(&e), "trial {trial}: {text}");
        }
    });
}

/// A legal tree at the bound whose deepest leaf is a negative literal:
/// does its rendering read back through the text door?
#[test]
fn review_negative_leaf_at_the_bound_reads_back() {
    use editor_core::unparse;
    on_stack(WASM_STACK, || {
        for levels in [1, 2, 127, 128] {
            let e = deep_length(-0.5, levels);
            let text = unparse(&e);
            let back = parse_expr(&text, &BTreeMap::new());
            eprintln!(
                "REVIEW negative leaf, {levels} levels: text starts {:?}; parse {}",
                &text[..text.len().min(24)],
                match &back {
                    Ok(b) => format!("ok, bit_eq {}", b.bit_eq(&e)),
                    Err(err) => format!("REFUSED: {err}"),
                }
            );
        }
    });
}
