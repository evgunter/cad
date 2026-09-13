//! **The roster: every predicate name this crate decides is routed to a
//! recourse sentence, or listed here as carrying none and why.**
//!
//! `PathError::Escalated`'s `Display` routes a recourse by matching the
//! escalated predicate's NAME. A name no arm carries falls through, and
//! the fall-through is the whole subject of this suite:
//!
//! - it must NAME the hole (`geom_core::MissingRecourse`) rather than
//!   assert a category over a name nobody classified — "path junction
//!   classification" is a true label for exactly two names and a false
//!   one for the rest;
//! - a name must not reach it by accident. The roster below is read out
//!   of the crate's own `src`, so a gate added anywhere — in `sugar.rs`,
//!   in `path/arc_fillet.rs`, in a `#[cfg(test)]` module — is as visible
//!   to these rows as one added beside a name they already know, and a
//!   gate RENAMED under a routed arm goes red the same way.
//!
//! **The reader is fail-loud about what it cannot read.** A `decide*`
//! call whose first argument is not a plain string literal does not get
//! skipped: its expression is recorded as an *indirect site* and must
//! appear in [`INDIRECT`], which says what carries the name there. The
//! carriers named there are then scanned for the literals they hold, so
//! a name that reaches the funnel through a parameter, a const or a
//! struct field is in the roster beside the ones written at the call.
//!
//! Which arm answers a name is MEASURED — each name is put through the
//! door's own error value and the rendered text read — never inferred
//! from the order of the arms in the source.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Indeterminate, MarginDiag, MissingRecourse, Tol};
use profile::PathError;
use std::collections::BTreeSet;

// ------------------------------------------------------------------ the reader

/// The `decide*` calls whose first argument the reader cannot read at
/// the site, and what carries the name there. Keyed `<file>: <expr>`,
/// which is what [`funnel_calls`] reports.
///
/// An entry's carrier is scanned for the literals it holds, so this
/// table is not a place a name can hide: it says WHERE to look, and the
/// looking is [`carried_names`]'s.
const INDIRECT: &[(&str, &str, &str)] = &[
    (
        "path/arc_fillet.rs: name",
        "travel",
        "the corner window's advance and reach gates take the name from \
         `FilletSide::travel`, which answers the straight carrier's name or the arc \
         name its caller passes",
    ),
    (
        "path/verbs.rs: name",
        "gate_positive",
        "`gate_positive` meters any authored magnitude against zero and reports under \
         the name its caller supplies",
    ),
    (
        "seg.rs: name",
        "coincident",
        "`seg::coincident` answers point-coincidence questions under the name its \
         caller supplies",
    ),
];

/// Every name this crate's `src` decides, and every `decide*` call the
/// reader could not read at the site.
fn funnel_calls() -> (BTreeSet<String>, BTreeSet<String>) {
    let src = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut names = BTreeSet::new();
    let mut indirect = BTreeSet::new();
    for path in test_utils::source::rust_sources(&src) {
        let text = std::fs::read_to_string(&path).expect("a readable source file");
        // The code-and-literals view blanks comments in place, so prose
        // that spells `decide (` is not a call site and the offsets
        // still index the source byte for byte.
        let code = test_utils::source::code_and_literals(&text);
        let rel = path
            .strip_prefix(&src)
            .expect("a file under the crate's src")
            .to_string_lossy()
            .replace('\\', "/");
        for open in call_sites(&code, "decide", true) {
            let arg = code[open..].trim_start();
            match plain_literal(arg) {
                Some(name) => {
                    names.insert(name.to_string());
                }
                None => {
                    let expr: String = arg
                        .chars()
                        .take_while(|c| *c != ',' && *c != ')')
                        .collect::<String>()
                        .trim()
                        .to_string();
                    indirect.insert(format!("{rel}: {expr}"));
                }
            }
        }
        for (_, carrier, _) in INDIRECT {
            for open in call_sites(&code, carrier, false) {
                for literal in group_literals(&code, open) {
                    names.insert(literal);
                }
            }
        }
    }
    (names, indirect)
}

/// The byte offsets just past each `token(` or `token {` in `code`.
///
/// `suffixed` admits a token whose name carries a suffix — the funnel's
/// `_flagged` and `_invariant` doors are `decide` calls too.
fn call_sites(code: &str, token: &str, suffixed: bool) -> Vec<usize> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(hit) = code[at..].find(token) {
        let start = at + hit;
        at = start + token.len();
        if start > 0
            && code[..start]
                .chars()
                .next_back()
                .is_some_and(|c| c == '_' || c.is_ascii_alphanumeric())
        {
            continue;
        }
        let rest = &code[at..];
        let tail = rest.trim_start();
        let skipped = rest.len() - tail.len();
        if suffixed {
            let suffix: String = rest.chars().take_while(|c| *c != '(').collect();
            if !suffix
                .chars()
                .all(|c| c == '_' || c.is_ascii_alphanumeric())
            {
                continue;
            }
            if let Some(i) = rest.find('(') {
                out.push(at + i + 1);
                at += i + 1;
            }
            continue;
        }
        if tail.starts_with('(') || tail.starts_with('{') {
            out.push(at + skipped + 1);
        }
    }
    out
}

/// The plain string literals inside the bracket group whose contents
/// start at `open` — the group that a [`Carrier::Call`] token opened.
fn group_literals(code: &str, open: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let bytes = code.as_bytes();
    let mut i = open;
    while i < bytes.len() {
        match bytes[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
            b'"' => {
                if let Some(name) = plain_literal(&code[i..]) {
                    out.push(name.to_string());
                    i += name.len() + 2;
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

/// The contents of a plain string literal at the head of `text`, if
/// that is what it opens with. A literal carrying an escape is not
/// plain and is not a predicate name.
fn plain_literal(text: &str) -> Option<&str> {
    let rest = text.strip_prefix('"')?;
    let end = rest.find('"')?;
    let body = &rest[..end];
    (!body.contains('\\')).then_some(body)
}

// ------------------------------------------------------------------ the roster

/// Every decided name with NO sentence of its own, and why it needs
/// none. A name here renders the gap sentence: the refusal says the
/// table records nothing for it, and the shared coincidence recourse
/// that rides the escalation's own payload is the lever at the site.
///
/// A name leaves this list the moment an arm claims it, and a new gate
/// joins it only by a hand that writes down the reason.
const UNROUTED: &[(&str, &str)] = &[
    // `seg.rs` — the segment and joint classifications. Eight of them
    // reach this door through the stored-form read and are routed
    // there; these six are decided for the validator, whose refusals
    // are typed on `ProfileError`, and the path door has no lever to
    // add at a segment coincidence.
    ("arc_apex_identity", SEG),
    ("arc_span", SEG),
    ("collinear_overlap", SEG),
    ("line_span", SEG),
    ("ray_advance", SEG),
    ("ray_side", SEG),
    // `validate.rs` — loop-level classifications, decided for
    // `ProfileError::Escalated` and its own Display.
    ("canonical_order_x", LOOP_LEVEL),
    ("canonical_order_y", LOOP_LEVEL),
    ("contact_at_shared_vertex", LOOP_LEVEL),
    ("loop_orientation", LOOP_LEVEL),
    // Authored magnitudes metered against zero.
    ("path_arc_center_radius", MAGNITUDE),
    ("path_arc_chord", MAGNITUDE),
    ("path_arc_sweep", MAGNITUDE),
    ("path_circle_radius", MAGNITUDE),
    ("path_director_norm", MAGNITUDE),
    ("path_fillet_radius", MAGNITUDE),
    // The carrier geometry the arc and fillet verbs solve against.
    ("path_arc_bulge", CARRIER),
    ("path_arc_center_equidistant", CARRIER),
    ("path_arc_continue_on_carrier", CARRIER),
    ("path_arc_via_offset", CARRIER),
    ("path_carrier_identity", CARRIER),
    ("path_carrier_meet", CARRIER),
    ("path_collinear_target", CARRIER),
    // Where the corner sits along a side's carrier.
    ("path_corner_advance", CORNER_WINDOW),
    ("path_corner_advance_arc", CORNER_WINDOW),
    ("path_corner_reach_arc", CORNER_WINDOW),
    ("path_corner_turn", CORNER_WINDOW),
    // The lever the seam's own two gates are metered through.
    (
        "path_seam_arrival_lever",
        "the magnitude the seam arrival's turn and side gates are levered by; those two \
         gates carry the site's sentence and are routed above",
    ),
];

const SEG: &str = "a `seg.rs` segment or joint classification, decided for the validator; the \
                   eight of them the stored-form read re-runs are routed above, and at a \
                   segment coincidence the shared recourse is the lever";
const LOOP_LEVEL: &str = "a `validate.rs` loop-level classification, whose escalation is typed \
                          on `ProfileError::Escalated` and rendered there";
const MAGNITUDE: &str = "an authored magnitude metered against zero: the lever is the number \
                         the caller supplied, which is what 'move the geometry' names here";
const CARRIER: &str = "a carrier-geometry classification of the arc and fillet verbs — where \
                       two carriers meet, whether they are the same one, which side of one \
                       a point falls; the lever is the shared coincidence recourse";
const CORNER_WINDOW: &str = "a corner-window gate: where the corner sits along a side's \
                             carrier. A corner outside the window is reported as a fact \
                             about the corner, and the in-band case has no lever beyond the \
                             shared one";

// ------------------------------------------------------------------ rows

/// The run's band, and an escalation carrying `name` inside it.
fn escalation(name: &'static str) -> Indeterminate {
    let band = Band::linear(Tol::witness()).expect("the run's band forms");
    Indeterminate {
        margin: MarginDiag::Value((band.zero() + band.escalate()) / 2.0),
        band,
        predicate: Some(name),
    }
}

/// Renders the door's refusal for an escalation under `name`.
fn rendered(name: &'static str) -> String {
    PathError::<f64>::Escalated {
        source: escalation(name),
    }
    .to_string()
}

/// Whether the refusal for `name` is the fall-through — measured by
/// reading the text, not by reading the match.
fn falls_through(name: &'static str) -> bool {
    rendered(name).contains(&MissingRecourse(Some(name)).to_string())
}

/// **Every name the crate decides is routed to a sentence, or listed
/// with the reason it carries none.**
///
/// The two sides are read from different places and compared: the names
/// out of the crate's `src`, the routing out of the rendered refusal.
/// A gate added anywhere is unrouted and unlisted, so it reds here; a
/// gate renamed under a routed arm stops being routed, so it reds here
/// too; and a name struck off `UNROUTED` without a sentence reds as a
/// stale entry.
#[test]
fn every_decided_name_is_routed_or_listed_with_its_reason() {
    let (names, _) = funnel_calls();
    assert!(
        names.contains("path_junction_turn") && names.contains("fillet_corner_turn"),
        "the reader found no funnel calls it should have: {names:?}"
    );
    let listed: BTreeSet<&str> = UNROUTED.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        listed.len(),
        UNROUTED.len(),
        "a name is listed twice in UNROUTED"
    );
    for (name, _) in UNROUTED {
        assert!(
            names.contains(*name),
            "`{name}` is listed as carrying no recourse, but nothing in the crate's src \
             decides it — a stale entry"
        );
    }
    for name in &names {
        let name: &'static str = Box::leak(name.clone().into_boxed_str());
        let unrouted = falls_through(name);
        assert_eq!(
            unrouted,
            listed.contains(name),
            "`{name}` is {} by the door and {} in UNROUTED; the refusal reads: {}",
            if unrouted { "unrouted" } else { "routed" },
            if listed.contains(name) {
                "listed"
            } else {
                "unlisted"
            },
            rendered(name)
        );
    }
}

/// **A name no arm carries renders the gap sentence — and no category.**
///
/// "path junction classification" is a claim about the two junction
/// keys. This row constructs an escalation under a name the crate does
/// not decide and reads what comes back: the hole, named, and no label
/// over it.
#[test]
fn an_unknown_name_names_the_hole_and_asserts_nothing() {
    let unknown = "roster_unknown_probe";
    let (names, _) = funnel_calls();
    assert!(
        !names.contains(unknown),
        "the probe name must be one the crate does not decide"
    );
    let text = rendered(unknown);
    assert!(
        text.contains(&MissingRecourse(Some(unknown)).to_string()),
        "the refusal names the hole: {text}"
    );
    assert!(
        !text.contains("path junction classification"),
        "the refusal asserts a category over a name nothing classified: {text}"
    );
    assert!(
        text.contains(unknown),
        "the refusal names the predicate that escalated: {text}"
    );
    // The two junction keys are the names that label IS true for, and
    // they keep it.
    for key in ["path_junction_turn", "path_junction_side"] {
        assert!(
            rendered(key).starts_with("path junction classification:"),
            "the junction keys keep their label: {}",
            rendered(key)
        );
    }
}

/// **The dispatch order is the one the site states, and no name sits in
/// two layers.**
///
/// The fillet family is asked first, through `fillet_recourse_for`.
/// Every other layer is a literal in a match pattern in `path.rs`, and
/// the compiler catches a name written into two of THOSE (an
/// unreachable pattern) — what it cannot see is a name in the map AND
/// in a pattern, where the map silently wins and the pattern arm is
/// dead. That is what this row reads: the two sides, from their own
/// sources, and their intersection is empty.
#[test]
fn the_dispatch_order_owns_each_name_in_exactly_one_layer() {
    let path_rs = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src/path.rs");
    let text = std::fs::read_to_string(&path_rs).expect("the path module is readable");
    let code = test_utils::source::code_and_literals(&text);
    let mut patterned: BTreeSet<String> = BTreeSet::new();
    for open in call_sites(&code, "Some", false) {
        // A pattern group is string literals and `|`; anything else
        // (`Some(predicate)`, `Some(policy)`) is not one.
        let group = group_literals(&code, open);
        for name in group {
            patterned.insert(name);
        }
    }
    assert!(
        patterned.contains("path_junction_turn") && patterned.contains("vertex_separation"),
        "the pattern reader found no arms it should have: {patterned:?}"
    );
    for name in &patterned {
        assert!(
            profile::fillet_recourse_for(name).is_none(),
            "`{name}` is written into a `path.rs` match pattern AND answered by \
             `fillet_recourse_for`. The map is asked first, so the pattern arm is dead: \
             one of the two is the name's home, not both"
        );
    }
    // The other direction, driven: every name the map answers renders
    // the fillet arm's sentence, so the map really is asked first.
    let (names, _) = funnel_calls();
    let mut answered: BTreeSet<&str> = BTreeSet::new();
    for name in &names {
        let name: &'static str = Box::leak(name.clone().into_boxed_str());
        let Some(sentence) = profile::fillet_recourse_for(name) else {
            continue;
        };
        answered.insert(name);
        let text = rendered(name);
        assert!(
            text.starts_with("resolving the fillet at this corner,") && text.contains(sentence),
            "`{name}` is answered by the map but does not render its sentence: {text}"
        );
    }
    let family: BTreeSet<&str> = names
        .iter()
        .filter(|n| n.starts_with("fillet_"))
        .map(String::as_str)
        .collect();
    assert_eq!(
        answered.len(),
        family.len(),
        "the map answers {answered:?}, and the crate decides the fillet family {family:?}"
    );
}

/// **Every `decide*` call the reader cannot read at the site is
/// declared, with what carries the name there.**
///
/// A name built somewhere other than the call — a parameter, a const, a
/// struct field, a `concat!` — is exactly the name a census misses.
/// This row makes missing one loud: the reader records the expression
/// instead of skipping it, and an expression nobody has written down
/// fails here rather than leaving a gate off the roster.
#[test]
fn every_indirect_funnel_call_is_declared() {
    let (_, indirect) = funnel_calls();
    let declared: BTreeSet<String> = INDIRECT
        .iter()
        .map(|(site, _, _)| site.to_string())
        .collect();
    assert_eq!(
        indirect, declared,
        "the funnel calls whose name the reader cannot read at the site are not the ones \
         declared in INDIRECT"
    );
}
