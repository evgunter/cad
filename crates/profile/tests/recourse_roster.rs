//! **The roster: every decision this crate makes is a [`Gate`], and each
//! door ends an undecided gate by an exhaustive match on it.**
//!
//! The table (`src/gate.rs`) is closed, so a gate with no ending or no
//! subject does not compile. What this suite holds is what the type
//! cannot: that no funnel call in `src` decides under a name written
//! anywhere but the table (the census reads no literal), that every row
//! is a gate some site raises, and that each kind ends the way D4 ¶1 (i)
//! says at every door.
//!
//! The reader is `test_utils::source::predicate_census`. **What it
//! cannot read it reports**: a spelling it does not know is
//! `unreadable`, a name-bearing argument that is not a plain literal is
//! `indirect`, and `include!`/`#[path]` come back in `unwalked`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::EndingKind;
use geom_core::{Band, Indeterminate, MarginDiag};
use profile::{EscalatedAt, EscalationSite, Gate, PathError, ProfileError, SegmentRef};

fn census() -> test_utils::source::PredicateCensus {
    test_utils::source::predicate_census(
        &test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src"),
        &[],
    )
}

/// The escalation of `gate` at `margin`, against the band `(1e-9, 1e-8)`.
fn at(gate: Gate, margin: MarginDiag) -> Indeterminate {
    Indeterminate {
        margin,
        band: Band::new(1e-9, 1e-8).expect("the band forms"),
        predicate: Some(gate.predicate()),
        terminal_sliver: false,
    }
}

fn door(site: EscalatedAt, gate: Gate, margin: MarginDiag) -> String {
    PathError::<f64>::Escalated {
        site,
        gate,
        source: at(gate, margin),
    }
    .to_string()
}

fn validation(site: EscalationSite, gate: Gate, margin: MarginDiag) -> String {
    ProfileError::Escalated {
        site,
        gate,
        source: at(gate, margin),
    }
    .to_string()
}

/// **No funnel call in `src` names its decision anywhere but the gate
/// table.** Every call the census finds takes a gate, or is the table's
/// own funnel handing a gate's name on (`Gate::read`, and `name` inside
/// the closures it runs), so
/// a `decide("…")` written at a site reds here as a literal name.
#[test]
fn no_funnel_call_in_src_takes_a_literal_name() {
    let census = census();
    assert!(
        census.names.is_empty(),
        "a funnel call decides under a literal name, not a gate: {:?}",
        census.names
    );
    let stray: Vec<&String> = census
        .indirect
        .iter()
        .filter(|site| {
            let (file, expr) = site.split_once(": ").expect("`<file>: <expr>`");
            let the_funnel = file == "gate.rs" && expr.starts_with("self.predicate(");
            !(the_funnel || expr == "name" || expr == "gate" || expr.starts_with("Gate::"))
        })
        .collect();
    assert!(
        stray.is_empty(),
        "a funnel call takes its name from something other than a gate: {stray:?}"
    );
    assert!(
        !census.indirect.is_empty(),
        "the reader found no funnel calls it should have"
    );
    assert!(census.unreadable.is_empty(), "{:?}", census.unreadable);
    assert!(census.unwalked.is_empty(), "{:?}", census.unwalked);
}

/// **Every row is a gate some production site raises, and names one
/// predicate.** The text read is the crate's production code alone:
/// comments and string literals blanked, every `#[cfg(test)]` item cut,
/// and the files `lib.rs` mounts only under test left out, with the
/// table itself not read. A gate spelled nowhere there is a stale row:
/// a raise site re-pointed to another gate, with the old one left in a
/// comment or a test, reds here.
#[test]
fn every_gate_is_raised_and_named_once() {
    let text = production_code();
    let mut names = std::collections::BTreeSet::new();
    for gate in Gate::ALL {
        assert!(
            names.insert(gate.predicate()),
            "`{}` names two gates",
            gate.predicate()
        );
        assert!(
            spelled(&text, &format!("Gate::{gate:?}")),
            "{gate:?} has a row and no production site raises it"
        );
        let subject = gate.subject();
        assert!(
            !subject.contains(": ") && !subject.contains('—') && subject == subject.to_lowercase(),
            "{gate:?}'s subject is not a plain clause that stands alone: {subject}"
        );
    }
}

/// Whether `needle` occurs in `code` as a whole path, not as the prefix
/// of a longer one (`Gate::ArcSweep` inside `Gate::ArcSweepRange`).
fn spelled(code: &str, needle: &str) -> bool {
    code.match_indices(needle)
        .any(|(at, _)| test_utils::source::boundary_after(code, at + needle.len()))
}

/// The crate's production code, as [`every_gate_is_raised_and_named_once`]
/// reads it.
fn production_code() -> String {
    let src = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let lib = test_utils::source::code_only(
        &std::fs::read_to_string(src.join("lib.rs")).expect("lib.rs is readable"),
    );
    let test_only: Vec<String> = test_utils::source::file_module_decls(&lib)
        .into_iter()
        .filter(|m| !test_utils::source::file_module_decls(&without_tests(&lib)).contains(m))
        .collect();
    assert!(
        test_only.iter().any(|m| m == "turn_rows"),
        "the reader found no test-only mount it should have: {test_only:?}"
    );
    let mut out = String::new();
    for file in test_utils::source::rust_sources(&src) {
        let stem = file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        if stem == "gate" || test_only.iter().any(|m| m == stem) {
            continue;
        }
        let text = std::fs::read_to_string(&file).expect("a src file is readable");
        out.push_str(&without_tests(&test_utils::source::code_only(&text)));
        out.push('\n');
    }
    out
}

/// `code` with every `#[cfg(test)]` item blanked.
fn without_tests(code: &str) -> String {
    let mut out = code.to_string();
    for (at, _) in code.match_indices("#[cfg(test)]") {
        if let Some(extent) = test_utils::source::attributed_extent(code, at) {
            let blank: String = code[extent.clone()]
                .chars()
                .map(|c| if c == '\n' { '\n' } else { ' ' })
                .collect();
            out.replace_range(extent, &blank);
        }
    }
    out
}

/// **A gate classified against the exact-order band has no tolerance
/// lever**: `Band::least()` does not move with ε, so its ending is the
/// lever alone.
#[test]
fn the_exact_order_gates_end_in_their_lever_alone() {
    for gate in [
        Gate::FilletLegFit,
        Gate::FilletLegReach,
        Gate::CanonicalOrderX,
        Gate::CanonicalOrderY,
    ] {
        assert!(
            matches!(gate.kind(), EndingKind::Lever(_)),
            "{gate:?} classifies against the exact-order band, where no tolerance decides it"
        );
    }
}

/// **No gate's lever names the tolerance.** Where a smaller ε decides a
/// margin the ending derives that arm, conditional and valued, from the
/// margin; a lever that says "lower the tolerance" says it on every
/// reading, decided or not.
#[test]
fn no_lever_names_the_tolerance() {
    let offenders: Vec<String> = Gate::ALL
        .iter()
        .filter(|gate| gate.kind().lever().contains("tolerance"))
        .map(|gate| format!("{gate:?}: {}", gate.kind().lever()))
        .collect();
    assert!(offenders.is_empty(), "{offenders:#?}");
}

/// **Both doors end an undecided gate as its kind warrants, at each
/// site.** A lever-only gate ends in its lever alone on a value and on a
/// straddle, noted on an unreadable margin; a sized one offers the
/// tolerance on a value, subdivision on a straddle and the inputs on an
/// unreadable margin; the stored-form site keeps the gate's kind under
/// the stored form's lever.
#[test]
fn the_undecided_endings_follow_the_gates_kind() {
    use EscalatedAt::{Door, StoredForm};
    let value = MarginDiag::value(5e-9);
    let straddle = MarginDiag::enclosure(-2e-9, 5e-9);
    let poison = MarginDiag::INVALID;
    let segment = SegmentRef {
        loop_index: 0,
        segment_index: 1,
    };
    let junction = "for tangency onto a new carrier use .tangent() (.tangent().tangent_arc_to(p) \
                    for an arc; the tangent-arc or seam-fillet close at the seam); for a straight \
                    continuation spell it line(len) off the directed point; otherwise move the \
                    geometry";
    let cases = [
        (
            "a junction's turn, value",
            door(Door, Gate::PathJunctionTurn, value),
            format!(
                "whether the path turns at this junction is undecided: margin 5e-9 lies inside \
                 the ambiguity band (1e-9, 1e-8). Recourse: {junction}"
            ),
        ),
        (
            "a junction's turn, straddle",
            door(Door, Gate::PathJunctionTurn, straddle),
            format!(
                "whether the path turns at this junction is undecided: enclosure [-2e-9, 5e-9] \
                 cannot be classified against the ambiguity band (1e-9, 1e-8). Recourse: \
                 {junction}"
            ),
        ),
        (
            "a junction's turn, unreadable",
            door(Door, Gate::PathJunctionTurn, poison),
            format!(
                "whether the path turns at this junction is undecided: margin is invalid (NaN \
                 or a refused enclosure) against the ambiguity band (1e-9, 1e-8). Recourse: \
                 {junction}; an unreadable margin may indicate a kernel bug worth reporting"
            ),
        ),
        (
            "a circle's radius, value",
            door(Door, Gate::PathCircleRadius, value),
            "whether a circle's radius is zero is undecided: margin 5e-9 lies inside the \
             ambiguity band (1e-9, 1e-8). Recourse: move the geometry, or, if this size is \
             intended, tighten the tolerance below 5e-10 m"
                .to_owned(),
        ),
        (
            "a circle's radius, straddle",
            door(Door, Gate::PathCircleRadius, straddle),
            "whether a circle's radius is zero is undecided: enclosure [-2e-9, 5e-9] cannot be \
             classified against the ambiguity band (1e-9, 1e-8). Recourse: subdivide the \
             parameter box for a tighter enclosure, or move the geometry"
                .to_owned(),
        ),
        (
            "a circle's radius, unreadable",
            door(Door, Gate::PathCircleRadius, poison),
            "whether a circle's radius is zero is undecided: margin is invalid (NaN or a refused \
             enclosure) against the ambiguity band (1e-9, 1e-8). Recourse: check the \
             operation's inputs upstream, then move the geometry; an unreadable margin may \
             indicate a kernel bug worth reporting"
                .to_owned(),
        ),
        (
            "a linear-band fillet gate, value",
            door(Door, Gate::FilletOffsetLineCircle, value),
            format!(
                "the fillet at this corner is undecided: margin 5e-9 lies inside the ambiguity \
                 band (1e-9, 1e-8). Recourse: {}, or, if this size is intended, tighten the \
                 tolerance below 5e-10 m",
                profile::FILLET_NO_CORNER_RECOURSE
            ),
        ),
        (
            "an exact-order fillet gate, value",
            door(Door, Gate::FilletLegReach, value),
            format!(
                "the fillet at this corner is undecided: margin 5e-9 lies inside the ambiguity \
                 band (1e-9, 1e-8). Recourse: {}",
                profile::FILLET_NO_CORNER_RECOURSE
            ),
        ),
        (
            "an exact-order fillet gate, unreadable",
            door(Door, Gate::FilletLegFit, poison),
            format!(
                "the fillet at this corner is undecided: margin is invalid (NaN or a refused \
                 enclosure) against the ambiguity band (1e-9, 1e-8). Recourse: {}; an \
                 unreadable margin may indicate a kernel bug worth reporting",
                profile::FILLET_FIT_RECOURSE
            ),
        ),
        (
            "the straight pair's corner turn, value",
            door(Door, Gate::PathCornerTurn, value),
            format!(
                "whether the two carriers turn at the corner is undecided: margin 5e-9 lies \
                 inside the ambiguity band (1e-9, 1e-8). Recourse: {}, or, if this size is \
                 intended, tighten the tolerance below 5e-10 m",
                profile::CORNER_ANCHOR_SEPARATION_LEVER
            ),
        ),
        (
            "a sized gate at the stored-form read, value",
            door(StoredForm, Gate::VertexSeparation, value),
            format!(
                "the fillet arc about to be stored is undecided: margin 5e-9 lies inside the \
                 ambiguity band (1e-9, 1e-8). Recourse: {}, or, if this size is intended, \
                 tighten the tolerance below 5e-10 m",
                profile::FILLET_STORED_FORM_INBAND_RECOURSE
            ),
        ),
        (
            "a lever gate at the stored-form read, unreadable",
            door(StoredForm, Gate::SegmentStraightness, poison),
            format!(
                "the fillet arc about to be stored is undecided: margin is invalid (NaN or a \
                 refused enclosure) against the ambiguity band (1e-9, 1e-8). Recourse: {}; an \
                 unreadable margin may indicate a kernel bug worth reporting",
                profile::FILLET_STORED_FORM_INBAND_RECOURSE
            ),
        ),
        (
            "validation, a loop's winding, straddle",
            validation(
                EscalationSite::Loop { loop_index: 0 },
                Gate::LoopOrientation,
                straddle,
            ),
            "on loop 0, which way round a loop runs is undecided: enclosure [-2e-9, 5e-9] \
             cannot be classified against the ambiguity band (1e-9, 1e-8). Recourse: move the \
             geometry"
                .to_owned(),
        ),
        (
            "validation, a segment's length, value",
            validation(
                EscalationSite::Segment(segment),
                Gate::VertexSeparation,
                value,
            ),
            "at loop 0 segment 1, whether a segment has any length is undecided: margin 5e-9 \
             lies inside the ambiguity band (1e-9, 1e-8). Recourse: move the geometry, or, if \
             this size is intended, tighten the tolerance below 5e-10 m"
                .to_owned(),
        ),
        (
            "validation, an arc's scene resolution, value",
            validation(
                EscalationSite::Segment(segment),
                Gate::ArcCarrierResolution,
                value,
            ),
            "at loop 0 segment 1, whether the scene can read an arc's difference from its \
             vertices at its magnitude is undecided: margin 5e-9 lies inside the ambiguity band \
             (1e-9, 1e-8). Recourse: move the profile nearer the origin, or author the arc with \
             a smaller radius (a flatter arc stores its centre farther out)"
                .to_owned(),
        ),
    ];
    for (what, got, want) in cases {
        assert_eq!(got, want, "{what}");
    }
}

/// **Each lever-kind row, and the sizes it was easy to mistake for one,
/// ends as its kind says at the validator's door**: the ending after the
/// payload on a value reading, written out per row so changing a row's
/// kind reds here.
#[test]
fn each_lever_row_ends_in_its_lever_on_a_value_reading() {
    const MOVE: &str = "Recourse: move the geometry";
    const SIZED: &str = "Recourse: move the geometry, or, if this size is intended, tighten the \
                         tolerance below 5e-10 m";
    let rows: &[(Gate, &str)] = &[
        (Gate::ArcSweepLength, MOVE),
        (Gate::ArcSweepRange, MOVE),
        (Gate::CanonicalOrderY, MOVE),
        (Gate::FullTurnSense, MOVE),
        (Gate::PathArcBulge, MOVE),
        (Gate::PathArcSweep, MOVE),
        (Gate::PathArcSweepFull, MOVE),
        (Gate::PathJunctionSide, MOVE),
        (Gate::SegmentStraightness, MOVE),
        (Gate::FullTurnReach, SIZED),
    ];
    for (gate, tail) in rows {
        let text = validation(
            EscalationSite::Loop { loop_index: 0 },
            *gate,
            MarginDiag::value(5e-9),
        );
        assert!(
            text.ends_with(&format!("(1e-9, 1e-8). {tail}")),
            "{gate:?} should end \"{tail}\": {text}"
        );
    }
}

/// **No lever ending advises a coarser tolerance**: D4 ¶1 (i) allows
/// that only where a refusal would otherwise name no recourse at all,
/// and every lever here names one (`docs/DESIGN.md`,
/// `[tolerance.recourse]`).
#[test]
fn no_lever_ending_advises_a_coarser_tolerance() {
    let loosening = [
        "coarser",
        "widen",
        "loosen",
        "larger tolerance",
        "raise the tolerance",
    ];
    let offenders: Vec<String> = Gate::ALL
        .iter()
        .filter(|gate| {
            let lever = gate.kind().lever();
            loosening.iter().any(|word| lever.contains(word))
        })
        .map(|gate| format!("{gate:?}: {}", gate.kind().lever()))
        .collect();
    assert!(offenders.is_empty(), "{offenders:#?}");
}
