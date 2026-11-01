//! **The roster: every predicate name this crate decides is answered by
//! the fillet table or has a row in `validate::DECISION_ENDINGS`, whose
//! kind says how its undecided refusal ends.**
//!
//! `PathError::Escalated`'s `Display` routes some names to sentences of
//! their own; the rest fall through, and `ProfileError::Escalated`
//! renders every name, both ending by the row's kind. This suite holds
//! the table complete both ways — a gate added or renamed anywhere in
//! `src` without a row reds, and so does a row nothing decides — and a
//! name with no row at runtime names its hole
//! (`geom_core::MissingRecourse`) at both doors.
//!
//! The reader is `test_utils::source::predicate_census`, the tree's one
//! home for this walk. **What it cannot read it reports**: a spelling
//! it does not know is `unreadable`, a name-bearing argument that is
//! not a plain literal is `indirect` and its carrier must be declared
//! below, and `include!`/`#[path]` — files it does not walk — come back
//! in `unwalked`. Every one of those is a red row here, so the roster's
//! completeness is a measurement rather than a claim.
//!
//! **The near-tangency addendum is keyed by name too, and it is a
//! note, not a recourse.** `ProfileError::Escalated` appends a site
//! NOTE for three carrier names at a segment pair; the levers ride the
//! row's ending for every name. Those three names also carry a sentence
//! at `PathError`'s stored-form arm, and the two differ on purpose — the
//! validator sees a segment pair the caller authored, the door a form
//! it is about to store.
//! `review_recourse_roster_r2_probes::the_validator_door_ends_by_the_row_and_notes_a_near_tangency`
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
    // `seg::junction_turn` reads a junction's turn under the name its
    // caller supplies: `path_junction_turn`, or the declared seam
    // arrival's `path_seam_arrival_turn`.
    NameCarrier::Call("junction_turn"),
    // `seg::joint_tangency` reads a joint's turn under the name its
    // caller supplies: validation's `path_junction_turn`, or the door's
    // stored-form read's `path_fillet_joint_turn`.
    NameCarrier::Call("joint_tangency"),
    // `seg::read_back` reads a constructed arc back at a point scalar
    // (`k_stats::check_unlogged`) under each read's name (`seg::START_READ`,
    // `seg::LANDING_READ`, `seg::THROUGH_POINT_READ`, `seg::BRANCH_READ`) and
    // its resolution gate's.
    NameCarrier::Call("read_back"),
    NameCarrier::Const("LANDING_READ"),
    NameCarrier::Const("THROUGH_POINT_READ"),
    NameCarrier::Const("START_READ"),
    NameCarrier::Const("BRANCH_READ"),
];

/// The `decide*` calls whose name the reader cannot read at the site,
/// as `<file>: <expr>`, with what carries the name there.
const INDIRECT: &[(&str, &str)] = &[
    (
        "path/arc_fillet.rs: name",
        "the advance and reach gates' `travel` result (two calls, one key)",
    ),
    ("path/verbs.rs: name", "`gate_positive`'s own parameter"),
    (
        "seg.rs: name",
        "`seg::coincident`'s and `seg::junction_turn`'s own parameter",
    ),
    (
        "seg.rs: ( turn_name, prev.heading_at(prev.b), next.heading_at(next.a), arm, TurnSense::Unsigned, band, )",
        "`seg::joint_tangency`'s `turn_name`, which its callers spell",
    ),
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
        margin: MarginDiag::value((band.zero() + band.escalate()) / 2.0),
        band,
        predicate: Some(name),
        terminal_sliver: false,
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

/// **The ending table and the crate's decided names are one set.**
///
/// Every name `src` decides is answered by `fillet_recourse_for` or has
/// exactly one row in `DECISION_ENDINGS`, never both; every row is a
/// name `src` decides, with a subject; and no name with a row renders
/// the gap sentence. Deleting a row, adding a gate without one, or
/// renaming a gate under one all red here.
#[test]
fn every_decided_name_has_one_row_and_every_row_is_decided() {
    let census = census();
    assert!(
        census.names.contains("path_junction_turn") && census.names.contains("fillet_corner_turn"),
        "the reader found no funnel calls it should have: {:?}",
        census.names
    );
    let rows: BTreeSet<&str> = profile::DECISION_ENDINGS.iter().map(|r| r.name).collect();
    assert_eq!(
        rows.len(),
        profile::DECISION_ENDINGS.len(),
        "a name has two rows in DECISION_ENDINGS"
    );
    for row in profile::DECISION_ENDINGS {
        assert!(
            census.names.contains(row.name),
            "`{}` has a row, but nothing in the crate's src decides it — a stale row",
            row.name
        );
        assert!(
            profile::decision_subject(row.name).is_some(),
            "`{}` has a row and no subject",
            row.name
        );
        assert!(
            profile::fillet_recourse_for(row.name).is_none(),
            "`{}` has a row and is answered by the fillet table: one home, not both",
            row.name
        );
    }
    for name in &census.names {
        let name: &'static str = Box::leak(name.clone().into_boxed_str());
        assert!(
            rows.contains(name) || profile::fillet_recourse_for(name).is_some(),
            "`{name}` is decided by this crate and has no row in \
             `validate::DECISION_ENDINGS`: give it one with its ending's kind and what its \
             margin measures. It renders: {}",
            rendered(name)
        );
        assert!(
            !names_a_gap(name),
            "`{name}` has a home and still names a gap: {}",
            rendered(name)
        );
    }
}

/// **Every name the crate decides has a subject a refusal can state.**
/// `validate::decision_subject` is the one home of the words a door puts
/// before a margin it could not call; a name missing from it renders
/// `UNNAMED_DECISION`, which the shape guard reads as no subject. The
/// fillet constructor's gates are the one exception: the path door
/// routes each to a sentence of its own, and validation never decides
/// them.
#[test]
fn every_decided_name_has_a_subject_or_a_sentence() {
    for name in &census().names {
        assert!(
            profile::decision_subject(name).is_some()
                || profile::fillet_recourse_for(name).is_some(),
            "`{name}` is decided by this crate and has no subject in \
             `validate::decision_subject`"
        );
        if let Some(subject) = profile::decision_subject(name) {
            assert!(
                !subject.contains(": ")
                    && !subject.contains('—')
                    && subject == subject.to_lowercase(),
                "`{name}`'s subject is not a plain clause that stands alone: {subject}"
            );
        }
    }
}

/// **A name no arm carries and `src` has not listed renders the gap
/// sentence — and no category.**
///
/// The junction subjects are claims about the two junction
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
        !text.contains("at this junction"),
        "the refusal asserts a category over a name nothing classified: {text}"
    );
    assert!(
        !text.contains(unknown),
        "the predicate's name is routing and stays out of the sentence: {text}"
    );
    // The validator's door names the same hole.
    let at_rest = profile::ProfileError::Escalated {
        site: profile::EscalationSite::Loop { loop_index: 0 },
        source: escalation(unknown),
    }
    .to_string();
    assert!(
        at_rest.contains(&MissingRecourse(Some(unknown)).to_string()),
        "the validator's door names the hole too: {at_rest}"
    );
    // The two junction keys are the names that label IS true for, and
    // they keep it.
    for key in ["path_junction_turn", "path_junction_side"] {
        let subject = profile::decision_subject(key).expect("a junction key has a subject");
        assert!(
            rendered(key).starts_with(subject),
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
        patterned.contains("path_leg_length") && patterned.contains("vertex_separation"),
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

/// The escalation under `name` at `margin`, against the band
/// `(1e-9, 1e-8)`.
fn at(name: &'static str, margin: MarginDiag) -> Indeterminate {
    Indeterminate {
        margin,
        band: Band::new(1e-9, 1e-8).expect("the band forms"),
        predicate: Some(name),
        terminal_sliver: false,
    }
}

/// **Both doors end an undecided decision as its kind warrants, and
/// the same way.** A lever-only decision (a junction's turn, a loop's
/// winding, the canonical order, an arc's scene resolution) ends in its
/// lever alone on a value and on a straddle, noted on an unreadable
/// margin; a sized one (a radius, a
/// segment's length) offers the tolerance on a value, subdivision on a
/// straddle and the inputs on an unreadable margin. A name nothing
/// decides keeps the sized ending and names the hole.
#[test]
fn the_undecided_endings_follow_the_decisions_kind() {
    use profile::{EscalationSite, ProfileError, SegmentRef};
    let value = MarginDiag::value(5e-9);
    let straddle = MarginDiag::enclosure(-2e-9, 5e-9);
    let poison = MarginDiag::INVALID;
    let path = |name, margin| {
        PathError::<f64>::Escalated {
            source: at(name, margin),
        }
        .to_string()
    };
    let validation = |site, name, margin| {
        ProfileError::Escalated {
            site,
            source: at(name, margin),
        }
        .to_string()
    };
    let segment = SegmentRef {
        loop_index: 0,
        segment_index: 1,
    };
    let cases = [
        (
            "a junction's turn, value",
            path("path_junction_turn", value),
            "whether the path turns at this junction is undecided: margin 5e-9 lies inside the \
             ambiguity band (1e-9, 1e-8). Recourse: for tangency onto a new carrier use .tangent() (.tangent().tangent_arc_to(p) for an \
             arc; the tangent-arc or seam-fillet close at the seam); for a straight continuation spell it line(len) off the directed point; \
             otherwise move the geometry",
        ),
        (
            "a junction's turn, straddle",
            path("path_junction_turn", straddle),
            "whether the path turns at this junction is undecided: enclosure [-2e-9, 5e-9] \
             cannot be classified against the ambiguity band (1e-9, 1e-8). Recourse: for tangency onto a new carrier use .tangent() (.tangent().tangent_arc_to(p) for an \
             arc; the tangent-arc or seam-fillet close at the seam); for a straight continuation spell it line(len) off the directed point; \
             otherwise move the geometry",
        ),
        (
            "a junction's turn, unreadable",
            path("path_junction_turn", poison),
            "whether the path turns at this junction is undecided: margin is invalid (NaN or a \
             refused enclosure) against the ambiguity band (1e-9, 1e-8). Recourse: for tangency \
             onto a new carrier use .tangent() (.tangent().tangent_arc_to(p) for an \
             arc; the tangent-arc or seam-fillet close at the seam); for a straight continuation spell it line(len) off the directed point; \
             otherwise move the geometry; an unreadable margin may indicate a kernel bug worth \
             reporting",
        ),
        (
            "a circle's radius, value",
            path("path_circle_radius", value),
            "whether a circle's radius is zero is undecided: margin 5e-9 lies inside the \
             ambiguity band (1e-9, 1e-8). Recourse: move the geometry, or, if this size is \
             intended, tighten the tolerance below 5e-10 m",
        ),
        (
            "a circle's radius, straddle",
            path("path_circle_radius", straddle),
            "whether a circle's radius is zero is undecided: enclosure [-2e-9, 5e-9] cannot be \
             classified against the ambiguity band (1e-9, 1e-8). Recourse: subdivide the \
             parameter box for a tighter enclosure, or move the geometry",
        ),
        (
            "a circle's radius, unreadable",
            path("path_circle_radius", poison),
            "whether a circle's radius is zero is undecided: margin is invalid (NaN or a refused \
             enclosure) against the ambiguity band (1e-9, 1e-8). Recourse: check the \
             operation's inputs upstream, then move the geometry; an unreadable margin may \
             indicate a kernel bug worth reporting",
        ),
        (
            "an unknown name, value",
            path("roster_unknown_probe", value),
            "an unnamed decision is undecided: margin 5e-9 lies inside the ambiguity band \
             (1e-9, 1e-8). Recourse: move the geometry, or, if this size is intended, tighten \
             the tolerance below 5e-10 m — no recourse is recorded for this decision",
        ),
        (
            "validation, a loop's winding, value",
            validation(
                EscalationSite::Loop { loop_index: 0 },
                "loop_orientation",
                value,
            ),
            "on loop 0, which way round a loop runs is undecided: margin 5e-9 lies inside the \
             ambiguity band (1e-9, 1e-8). Recourse: move the geometry",
        ),
        (
            "validation, a loop's winding, straddle",
            validation(
                EscalationSite::Loop { loop_index: 0 },
                "loop_orientation",
                straddle,
            ),
            "on loop 0, which way round a loop runs is undecided: enclosure [-2e-9, 5e-9] \
             cannot be classified against the ambiguity band (1e-9, 1e-8). Recourse: move the \
             geometry",
        ),
        (
            "validation, a loop's winding, unreadable",
            validation(
                EscalationSite::Loop { loop_index: 0 },
                "loop_orientation",
                poison,
            ),
            "on loop 0, which way round a loop runs is undecided: margin is invalid (NaN or a \
             refused enclosure) against the ambiguity band (1e-9, 1e-8). Recourse: move the \
             geometry; an unreadable margin may indicate a kernel bug worth reporting",
        ),
        (
            "validation, a segment's length, value",
            validation(EscalationSite::Segment(segment), "vertex_separation", value),
            "at loop 0 segment 1, whether a segment has any length is undecided: margin 5e-9 \
             lies inside the ambiguity band (1e-9, 1e-8). Recourse: move the geometry, or, if \
             this size is intended, tighten the tolerance below 5e-10 m",
        ),
        (
            "validation, a segment's length, straddle",
            validation(
                EscalationSite::Segment(segment),
                "vertex_separation",
                straddle,
            ),
            "at loop 0 segment 1, whether a segment has any length is undecided: enclosure \
             [-2e-9, 5e-9] cannot be classified against the ambiguity band (1e-9, 1e-8). \
             Recourse: subdivide the parameter box for a tighter enclosure, or move the geometry",
        ),
        (
            "validation, a segment's length, unreadable",
            validation(
                EscalationSite::Segment(segment),
                "vertex_separation",
                poison,
            ),
            "at loop 0 segment 1, whether a segment has any length is undecided: margin is \
             invalid (NaN or a refused enclosure) against the ambiguity band (1e-9, 1e-8). \
             Recourse: check the operation's inputs upstream, then move the geometry; an \
             unreadable margin may indicate a kernel bug worth reporting",
        ),
        (
            "validation, the canonical order, value",
            validation(
                EscalationSite::Loop { loop_index: 0 },
                "canonical_order_x",
                value,
            ),
            "on loop 0, which of two points comes first along x is undecided: margin 5e-9 lies \
             inside the ambiguity band (1e-9, 1e-8). Recourse: move the geometry",
        ),
        (
            "validation, an arc's scene resolution, value",
            validation(
                EscalationSite::Segment(segment),
                "arc_carrier_resolution",
                value,
            ),
            "at loop 0 segment 1, whether the scene can read an arc's difference from its \
             vertices at its magnitude is undecided: margin 5e-9 lies inside the ambiguity band \
             (1e-9, 1e-8). Recourse: move the profile nearer the origin, or author the arc with \
             a smaller radius (a flatter arc stores its centre farther out)",
        ),
        (
            "validation, the junction's side, value",
            validation(
                EscalationSite::Segment(segment),
                "path_junction_side",
                value,
            ),
            "at loop 0 segment 1, whether the path carries straight on at this junction or \
             doubles back into a cusp is undecided: margin 5e-9 lies inside the ambiguity band \
             (1e-9, 1e-8). Recourse: move the geometry",
        ),
        (
            "the junction's side at the path door, value",
            path("path_junction_side", value),
            "whether the path carries straight on at this junction or doubles back into a cusp \
             is undecided: margin 5e-9 lies inside the ambiguity band (1e-9, 1e-8). Recourse: \
             move the geometry",
        ),
        (
            "validation, an unknown name, value",
            validation(
                EscalationSite::Loop { loop_index: 0 },
                "roster_unknown_probe",
                value,
            ),
            "on loop 0, an unnamed decision is undecided: margin 5e-9 lies inside the ambiguity \
             band (1e-9, 1e-8). Recourse: move the geometry, or, if this size is intended, \
             tighten the tolerance below 5e-10 m — no recourse is recorded for this decision",
        ),
    ];
    for (what, got, want) in cases {
        assert_eq!(got, want, "{what}");
    }
}

/// **Each lever-kind row, and the size it was easy to mistake for one,
/// ends as its kind says at the validator's door**: the ending after the
/// payload on a value reading, written out per row so changing a row's
/// kind reds here.
#[test]
fn each_lever_row_ends_in_its_lever_on_a_value_reading() {
    const MOVE: &str = "Recourse: move the geometry";
    const SIZED: &str = "Recourse: move the geometry, or, if this size is intended, tighten the \
                         tolerance below 5e-10 m";
    let rows: &[(&str, &str)] = &[
        ("arc_sweep_length", MOVE),
        ("arc_sweep_range", MOVE),
        ("canonical_order_y", MOVE),
        ("full_turn_sense", MOVE),
        ("path_arc_bulge", MOVE),
        ("path_arc_sweep", MOVE),
        ("path_arc_sweep_full", MOVE),
        ("path_corner_turn", MOVE),
        ("path_seam_arrival_side", MOVE),
        ("path_seam_arrival_turn", MOVE),
        ("segment_straightness", MOVE),
        ("full_turn_reach", SIZED),
    ];
    for (name, tail) in rows {
        let text = profile::ProfileError::Escalated {
            site: profile::EscalationSite::Loop { loop_index: 0 },
            source: at(name, MarginDiag::value(5e-9)),
        }
        .to_string();
        assert!(
            text.ends_with(&format!("(1e-9, 1e-8). {tail}")),
            "`{name}` should end \"{tail}\": {text}"
        );
    }
}

/// **No lever ending advises a coarser tolerance.** Every decision row
/// whose ending is a lever (`EndingKind::Lever`: no size the user
/// chose) names the geometry's lever, and none advises loosening ε:
/// D4 ¶1 (i) allows that only as a last resort where a refusal would
/// otherwise name no recourse at all, and every lever here names one
/// (`docs/DESIGN.md`, `[tolerance.recourse]`).
#[test]
fn no_lever_ending_advises_a_coarser_tolerance() {
    let loosening = [
        "coarser",
        "widen",
        "loosen",
        "larger tolerance",
        "raise the tolerance",
    ];
    let offenders: Vec<String> = profile::DECISION_ENDINGS
        .iter()
        .filter_map(|row| match row.kind {
            geom_core::EndingKind::Lever(text) => loosening
                .iter()
                .any(|word| text.contains(word))
                .then(|| format!("{}: {text}", row.name)),
            geom_core::EndingKind::Sized => None,
        })
        .collect();
    assert!(offenders.is_empty(), "{offenders:#?}");
}
