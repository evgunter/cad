//! **The roster: every predicate name this crate decides is read, and
//! every site the reader cannot read is listed with what carries its
//! name.**
//!
//! The reader is `test_utils::source::predicate_census`, the tree's one
//! home for this walk, and the checks every roster makes of it are
//! `test_utils::source::roster_violations` — the harness every crate's
//! roster runs, and the one the tree-wide coverage row
//! (`test-utils/tests/predicate_name_coverage.rs`) asks a deciding crate
//! for. It reads a name at a `decide*` call, or out of a carrier
//! declared in [`CARRIERS`]. **What it cannot read it reports**, and
//! each is a red row here: a site whose name is a `const` this file does
//! not declare, an indirect site not in [`INDIRECT`], a spelling it does
//! not know, a file it does not walk, and a declared carrier that no
//! longer matches anything.
//!
//! A funnel that hands on the name its caller gives it is spelled so no
//! other call shares its token — `*_as`, "decided as" the row it is
//! handed — and declared in [`CARRIERS`], so its callers' names are read.
//! No door table: editor-core's names reach the user through its own
//! refusal sentences, not a door's error type.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source::{NameCarrier, crate_dir, predicate_census, roster_violations};

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
];

const FORWARDS: &str = "a funnel handing on the name its caller gives it, read where its callers \
                        call a declared carrier; one entry stands for every such site in the \
                        file, so a new forwarder under an undeclared token here is not seen";
const CLOSED_TYPE: &str = "a closed type's `name()`, whose names are spelled in its own match, \
                           which the census does not read";
const OTHER_CRATE: &str = "a `const` another crate declares, read by that crate's roster";
const NOT_A_NAME: &str = "another `decide*` call, whose first argument is not a name";

/// The indirect sites, as `<file>: <expr>`, with what carries the name.
const INDIRECT: &[(&str, &str)] = &[
    ("checks/at_rest.rs: name", FORWARDS),
    ("checks/at_rest.rs: site", FORWARDS),
    ("coincide/replay.rs: name", FORWARDS),
    ("eval/measure.rs: predicate", FORWARDS),
    ("eval/wire.rs: decision.name()", CLOSED_TYPE),
    (
        "eval/wire.rs: topo::clearance::CLEARANCE_MARGIN",
        OTHER_CRATE,
    ),
    ("eval/wire.rs: value", NOT_A_NAME),
    ("measure.rs: measured", NOT_A_NAME),
    ("measure.rs: ten", NOT_A_NAME),
    ("names/emit_topo.rs: predicate", FORWARDS),
    ("trace.rs: Margin::levered(", NOT_A_NAME),
    ("trace.rs: Margin::of((origin - at).dot(z))", NOT_A_NAME),
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
