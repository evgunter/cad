//! **The roster: every predicate name this crate decides at a call is
//! read, and every site the reader cannot read is listed with what
//! carries its name.**
//!
//! The reader is `test_utils::source::predicate_census`, the tree's one
//! home for this walk, and the checks every roster makes of it are
//! `test_utils::source::roster_violations` — the harness every crate's
//! roster runs, and the one the tree-wide coverage row
//! (`test-utils/tests/predicate_name_coverage.rs`) asks a deciding crate
//! for. It reads a name at a `decide*` call, or out of a carrier
//! declared in [`CARRIERS`]. **What it cannot read it reports**, and
//! each is a red row here: a site whose name is a `const` this file does
//! not declare, an indirect site not in [`INDIRECT`] (keyed by the `fn`
//! it stands in, `<file>: <fn>: <head>`, so each forwarder is an entry
//! by name and a new one reds until it is declared), a spelling it does
//! not know, a file it does not walk, and a declared carrier that no
//! longer matches anything.
//!
//! A funnel that hands on the name its caller gives it is spelled so no
//! other call shares its token — `*_as`, "decided as" the row it is
//! handed — and declared in [`CARRIERS`], so its callers' names are read:
//! measure's `parallel_as`, the `zero_as` closures, at_rest's
//! `gap_sign_as`/`measure_sign_as`, and the topological naming's carrier
//! reads (`on_line_as`, `place_as`, `sign_as`, `offset_as`, `span_as`,
//! `stretches_as`, `holds_as`, `cover_as`, `runs_with_as`,
//! `edge_holds_as`), whose name is not their first argument.
//!
//! **Not yet complete.** editor-core's own closed `decisions!` table
//! (`decision.rs`) is decided through `.name()` ([`CLOSED_TYPE`]); a row
//! spelled only there (`revolve_full_vs_partial`) is read by no census
//! until `work/onecopy/the-census-reads-no-closed-type-name-table.md`
//! lands.
//!
//! No door table: editor-core's names reach the user through its own
//! refusal sentences, not a door's error type.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source::{FORWARDS, NameCarrier, crate_dir, predicate_census, roster_violations};

/// The carriers that hand a name to a funnel from somewhere other than
/// the call.
const CARRIERS: &[NameCarrier] = &[
    // A name spelled in a `const` and handed to a funnel by it.
    NameCarrier::Const("ASSERT_BOUND"),
    NameCarrier::Const("AT_REST_QUIET_AXES_PARALLEL"),
    NameCarrier::Const("AT_REST_QUIET_BOUND"),
    NameCarrier::Const("AT_REST_QUIET_CENTRED"),
    NameCarrier::Const("AT_REST_QUIET_CONTACT_BOUND"),
    NameCarrier::Const("AXES_PARALLEL"),
    NameCarrier::Const("CHORD_ON_RIM"),
    NameCarrier::Const("EVAL_DIRECTION_NORM"),
    NameCarrier::Const("GAP_AXIS_PARALLEL_TO_PLANE"),
    NameCarrier::Const("GAP_ROUND_LARGER"),
    NameCarrier::Const("GAP_ROUND_SIDE"),
    NameCarrier::Const("MATE_READ_SLIDE_WITHIN"),
    NameCarrier::Const("MATE_SLIDES_INDEPENDENT"),
    NameCarrier::Const("ONE_CYLINDER"),
    NameCarrier::Const("ONE_CYLINDER_AXIS"),
    NameCarrier::Const("ON_MEMBER_EDGE"),
    NameCarrier::Const("SEL_PLANE_DISTANCE"),
    NameCarrier::Const("TRACED_FRAME_SQUARE"),
    NameCarrier::Const("TRACE_REACHES"),
    // The funnels that hand on their callers' names.
    NameCarrier::Call("parallel_as"),
    NameCarrier::Call("zero_as"),
    // The topological naming's carrier reads and at_rest's measure signs,
    // each handing on its caller's name.
    NameCarrier::Call("ahead_as"),
    NameCarrier::Call("on_line_as"),
    NameCarrier::Call("place_as"),
    NameCarrier::Call("sign_as"),
    NameCarrier::Call("offset_as"),
    NameCarrier::Call("span_as"),
    NameCarrier::Call("stretches_as"),
    NameCarrier::Call("holds_as"),
    NameCarrier::Call("cover_as"),
    NameCarrier::Call("runs_with_as"),
    NameCarrier::Call("edge_holds_as"),
    NameCarrier::Call("gap_sign_as"),
    NameCarrier::Call("measure_sign_as"),
];

const CLOSED_TYPE: &str = "a closed type's `name()`, whose names are spelled in its own match, \
                           which the census does not read";
const OTHER_CRATE: &str = "a `const` another crate declares, read by that crate's roster";
const NOT_FIRST: &str = "a declared carrier's call whose name is not its first argument, named \
                         here by that first argument: a forwarder handing on its caller's name, \
                         or a caller whose literal is read";
const NOT_A_NAME: &str = "another `decide*` call, whose first argument is not a name";

/// The indirect sites, as `<file>: <expr>`, with what carries the name.
const INDIRECT: &[(&str, &str)] = &[
    (
        "checks/at_rest.rs: constant_pairing::zero_as: name",
        FORWARDS,
    ),
    (
        "checks/at_rest.rs: gap_sign_as: crate::measure::MeasureVerb::Gap",
        NOT_FIRST,
    ),
    ("checks/at_rest.rs: measure_sign_as: site", FORWARDS),
    ("coincide/replay.rs: one_cylinder::zero_as: name", FORWARDS),
    ("eval/measure.rs: parallel_as: predicate", FORWARDS),
    ("eval/wire.rs: turns_off: decision.name()", CLOSED_TYPE),
    ("eval/wire.rs: wire_assertion: value", NOT_A_NAME),
    (
        "eval/wire.rs: wire_clearance: topo::clearance::CLEARANCE_MARGIN",
        OTHER_CRATE,
    ),
    ("measure.rs: decide: measured", NOT_A_NAME),
    (
        "measure.rs: equal_decides_zero_both_signs_and_the_band: ten",
        NOT_A_NAME,
    ),
    ("names/emit_topo.rs: ahead_as: predicate", FORWARDS),
    ("names/emit_topo.rs: cover_as: &arc", NOT_FIRST),
    ("names/emit_topo.rs: cover_as: p", NOT_FIRST),
    ("names/emit_topo.rs: cover_as: p - q", NOT_FIRST),
    ("names/emit_topo.rs: cover_as: p0", NOT_FIRST),
    ("names/emit_topo.rs: cover_as: p1", NOT_FIRST),
    ("names/emit_topo.rs: edge_holds_as: p", NOT_FIRST),
    ("names/emit_topo.rs: holds_as: p", NOT_FIRST),
    ("names/emit_topo.rs: holds_as: per - u", NOT_FIRST),
    ("names/emit_topo.rs: holds_as: self.len - u", NOT_FIRST),
    ("names/emit_topo.rs: holds_as: u", NOT_FIRST),
    ("names/emit_topo.rs: offset_as::gap: predicate", FORWARDS),
    ("names/emit_topo.rs: on_line_as: predicate", FORWARDS),
    ("names/emit_topo.rs: place_as: p", NOT_FIRST),
    ("names/emit_topo.rs: place_as::sign: predicate", FORWARDS),
    ("names/emit_topo.rs: runs_with_as: ahead", NOT_FIRST),
    ("names/emit_topo.rs: runs_with_as: b", NOT_FIRST),
    (
        "names/emit_topo.rs: runs_with_as: other.carrier.eval(t)",
        NOT_FIRST,
    ),
    ("names/emit_topo.rs: runs_with_as: other.q0", NOT_FIRST),
    ("names/emit_topo.rs: runs_with_as: other.q1", NOT_FIRST),
    ("names/emit_topo.rs: runs_with_as: predicate", FORWARDS),
    ("names/emit_topo.rs: sign_as: predicate", FORWARDS),
    ("names/emit_topo.rs: span_as: a1 - a0", NOT_FIRST),
    ("names/emit_topo.rs: span_as: arc.mid", NOT_FIRST),
    ("names/emit_topo.rs: span_as: arc.p0", NOT_FIRST),
    ("names/emit_topo.rs: span_as: arc.p1", NOT_FIRST),
    ("names/emit_topo.rs: span_as: d1 - dm", NOT_FIRST),
    ("names/emit_topo.rs: stretches_as: arc", NOT_FIRST),
    ("trace.rs: square: Margin::levered(", NOT_A_NAME),
    (
        "trace.rs: square: Margin::of((origin - at).dot(z))",
        NOT_A_NAME,
    ),
];

/// **The census meets the harness every roster shares**, and reads
/// names, so the row does not pass over an empty walk.
#[test]
fn every_decided_name_is_read_and_every_site_is_listed() {
    let src = crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let census = predicate_census(&src, CARRIERS);
    assert!(
        !census.names.is_empty(),
        "the census read no name in {}",
        src.display()
    );
    let violations = roster_violations(&src, &census, CARRIERS, Some(INDIRECT));
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
