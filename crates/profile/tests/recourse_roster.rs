//! **The roster: every predicate name this crate decides is routed to a
//! recourse sentence, or listed in `validate::SHARED_CLAUSE_ONLY` as
//! having nothing beyond the shared clause, with what it measures.**
//!
//! `PathError::Escalated`'s `Display` routes a recourse by matching the
//! escalated predicate's NAME. A name no arm carries falls through, and
//! the fall-through is the subject of this suite:
//!
//! - a name the crate has DECIDED needs only the shared coincidence
//!   clause renders with that clause and nothing about a gap — the
//!   decision lives in `src`, beside the names, so the door can consult
//!   it and a reader of a real refusal is not told the table is broken;
//! - anything else renders `geom_core::MissingRecourse`, which names
//!   the hole rather than asserting a category;
//! - and a name cannot reach either by accident. The roster is read out
//!   of the crate's own `src`, so a gate added anywhere — in
//!   `sugar.rs`, in `path/arc_fillet.rs`, in a `#[cfg(test)]` module —
//!   is as visible as one added beside a name it already knows, and a
//!   gate RENAMED under a routed arm goes red the same way.
//!
//! The reader is `test_utils::source::predicate_census`, the tree's one
//! home for this walk. **What it cannot read it reports**: a spelling
//! it does not know is `unreadable`, a name-bearing argument that is
//! not a plain literal is `indirect` and its carrier must be declared
//! below, and `include!`/`#[path]` — files it does not walk — come back
//! in `unwalked`. Every one of those is a red row here, so the roster's
//! completeness is a measurement rather than a claim.
//!
//! **The crate holds a second predicate-keyed table, and it is not a
//! recourse table.** `ProfileError::Escalated`'s near-tangency addendum
//! (`validate.rs`) appends a site NOTE for three carrier names at a
//! segment pair and appends nothing otherwise; the levers themselves
//! ride `{source}` for every name, so its default asserts nothing and
//! it has no gap to name. Those three names also carry a sentence at
//! `PathError`'s stored-form arm, and the two differ on purpose — the
//! validator sees a segment pair the caller authored, the door a form
//! it is about to store. Both sites say so;
//! `review_recourse_roster_r2_probes::the_validator_door_appends_a_site_note_and_routes_nothing`
//! pins the addendum's key, `fillet_recourse_followability` the other.
//!
//! Which arm answers a name is MEASURED — each name is put through the
//! door's own error value and the rendered text read — never inferred
//! from the order of the arms in the source.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Indeterminate, MarginDiag, MissingRecourse, Tol};
use profile::PathError;
use std::collections::BTreeSet;
use test_utils::source::{NameCarrier, PredicateCensus, predicate_census};

/// The carriers that hand a name to the funnel from somewhere other
/// than the call, and the sites they answer for.
///
/// Declaring a carrier puts its own call sites in the census, so a name
/// written at one of them is rostered. A call the reader cannot resolve
/// to a literal stays in `indirect` until its carrier is declared here
/// — which is what makes a wrapper around a carrier a red row rather
/// than a silent hole.
pub(crate) const CARRIERS: &[NameCarrier] = &[
    // The corner window's advance and reach gates take the name from
    // `FilletSide::travel`, which answers the straight carrier's name
    // or the arc name its caller passes.
    NameCarrier::Call("travel"),
    // `gate_positive` meters any authored magnitude against zero under
    // the name its caller supplies.
    NameCarrier::Call("gate_positive"),
    // `seg::coincident` answers point-coincidence questions under the
    // name its caller supplies.
    NameCarrier::Call("coincident"),
];

/// The `decide*` calls whose name the reader cannot read at the site,
/// as `<file>: <expr>`, with what carries the name there.
const INDIRECT: &[(&str, &str)] = &[
    (
        "path/arc_fillet.rs: name",
        "the advance and reach gates' `travel` result (two calls, one key)",
    ),
    ("path/verbs.rs: name", "`gate_positive`'s own parameter"),
    ("seg.rs: name", "`seg::coincident`'s own parameter"),
];

fn census() -> PredicateCensus {
    predicate_census(
        &test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src"),
        CARRIERS,
    )
}

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

/// Whether the refusal for `name` carries the gap sentence — measured
/// by reading the text, not by reading the match.
fn names_a_gap(name: &'static str) -> bool {
    rendered(name).contains(&MissingRecourse(Some(name)).to_string())
}

/// **Every name the crate decides is routed to a sentence, or listed in
/// `src` as having nothing beyond the shared clause.**
///
/// The two sides are read from different places and compared: the names
/// out of the crate's `src`, the routing out of the rendered refusal.
/// A gate added anywhere is neither routed nor listed, so it reds here;
/// a gate renamed under a routed arm stops being routed, so it reds
/// here too; a name struck off the src list without a sentence reds as
/// a gap the door now names; and a stale entry — a listed name nothing
/// decides any more — reds as well.
#[test]
fn every_decided_name_is_routed_or_listed_with_its_reason() {
    let census = census();
    assert!(
        census.names.contains("path_junction_turn") && census.names.contains("fillet_corner_turn"),
        "the reader found no funnel calls it should have: {:?}",
        census.names
    );
    let listed: BTreeSet<&str> = profile::SHARED_CLAUSE_ONLY
        .iter()
        .map(|(n, _)| *n)
        .collect();
    assert_eq!(
        listed.len(),
        profile::SHARED_CLAUSE_ONLY.len(),
        "a name is listed twice in SHARED_CLAUSE_ONLY"
    );
    for (name, _) in profile::SHARED_CLAUSE_ONLY {
        assert!(
            census.names.contains(*name),
            "`{name}` is listed as needing nothing beyond the shared clause, but nothing in \
             the crate's src decides it — a stale entry"
        );
    }
    for name in &census.names {
        let name: &'static str = Box::leak(name.clone().into_boxed_str());
        let gap = names_a_gap(name);
        assert!(
            !gap,
            "`{name}` is decided by this crate and its refusal names a gap. Route it to a \
             sentence, or add it to `validate::SHARED_CLAUSE_ONLY` with what its margin \
             measures. It renders: {}",
            rendered(name)
        );
        // A listed name must reach the door's shared-clause arm, not a
        // routed sentence: the list is a claim about which arm answers.
        if listed.contains(name) {
            assert!(
                rendered(name).starts_with("escalated at the path door:"),
                "`{name}` is listed as shared-clause-only but a routed arm answers it: {}",
                rendered(name)
            );
        }
    }
}

/// **A name no arm carries and `src` has not listed renders the gap
/// sentence — and no category.**
///
/// "path junction classification" is a claim about the two junction
/// keys. This row constructs an escalation under a name the crate does
/// not decide and reads what comes back: the hole, named, and no label
/// over it.
#[test]
fn an_unknown_name_names_the_hole_and_asserts_nothing() {
    let unknown = "roster_unknown_probe";
    let census = census();
    assert!(
        !census.names.contains(unknown),
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

/// **Each name is owned by exactly one layer of the dispatch.**
///
/// The name says order because the order is what the ownership is FOR:
/// the fillet map is asked first, so a name in the map and in a
/// `path.rs` match pattern would render the map's sentence and leave
/// the pattern arm dead. What the row measures is that no name is in
/// both — disjointness, from the two sources themselves. The compiler
/// catches a name written into two `path.rs` patterns (an unreachable
/// pattern); it cannot see the map, which is the half this covers.
#[test]
fn the_dispatch_order_owns_each_name_in_exactly_one_layer() {
    let path_rs = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src/path.rs");
    let text = std::fs::read_to_string(&path_rs).expect("the path module is readable");
    let code = test_utils::source::code_and_literals(&text);
    let mut patterned: BTreeSet<String> = BTreeSet::new();
    let mut at = 0usize;
    while let Some(hit) = code[at..].find("Some(") {
        let open = at + hit + "Some".len();
        at = open + 1;
        if !test_utils::source::boundary_before(&code, at - "Some(".len()) {
            continue;
        }
        let Some(end) = test_utils::source::balanced_end(&code, open) else {
            continue;
        };
        // A pattern group is string literals and `|`; anything else
        // (`Some(predicate)`, `Some(policy)`) holds no literal.
        for piece in code[open + 1..end].split('|') {
            if let Some(name) = test_utils::source::plain_string_literal(piece) {
                patterned.insert(name.to_string());
            }
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
        assert!(
            profile::shared_clause_only(name).is_none(),
            "`{name}` is routed by a `path.rs` arm AND listed as shared-clause-only: the \
             arm answers it, so the list entry is false"
        );
    }
    // The other direction, driven: every name the map answers renders
    // the fillet arm's sentence, so the map really is asked first.
    let census = census();
    let mut answered: BTreeSet<&str> = BTreeSet::new();
    for name in &census.names {
        let name: &'static str = Box::leak(name.clone().into_boxed_str());
        let Some(sentence) = profile::fillet_recourse_for(name) else {
            continue;
        };
        answered.insert(name);
        let text = rendered(name);
        assert!(
            text.starts_with("the fillet at this corner is undecided:") && text.contains(sentence),
            "`{name}` is answered by the map but does not render its sentence: {text}"
        );
    }
    let family: BTreeSet<&str> = census
        .names
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

/// **Nothing in the crate's `src` is invisible to the reader.**
///
/// A name built somewhere other than the call — a parameter, a const, a
/// struct field, a `concat!` — is exactly the name a census misses, and
/// two hand-rolled readers were defeated that way before this one
/// existed. So the reader reports rather than skips, and each kind of
/// report is a red row here: an indirect site whose carrier nobody has
/// declared, a spelling it cannot parse, and a file it does not walk.
#[test]
fn nothing_in_src_is_invisible_to_the_reader() {
    let census = census();
    let declared: BTreeSet<String> = INDIRECT.iter().map(|(site, _)| site.to_string()).collect();
    assert_eq!(
        census.indirect, declared,
        "the funnel calls whose name the reader cannot read at the site are not the ones \
         declared in INDIRECT"
    );
    assert!(
        census.unreadable.is_empty(),
        "a `decide*` call is spelled in a way the reader cannot parse, so the gate it names \
         is outside the roster: {:?}",
        census.unreadable
    );
    assert!(
        census.unwalked.is_empty(),
        "an `include!` or `#[path]` pulls source into this crate from a file the reader does \
         not walk, so a gate there is outside the roster: {:?}",
        census.unwalked
    );
}
