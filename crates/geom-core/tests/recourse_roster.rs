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
//! not declare, an indirect site not in [`INDIRECT`] (keyed by the `fn`
//! it stands in, `<file>: <fn>: <head>`, so each forwarder is an entry
//! by name and a new one reds until it is declared), a spelling it does
//! not know, a file it does not walk, and a declared carrier that no
//! longer matches anything.
//!
//! A funnel that hands on the name its caller gives it is spelled so no
//! other call shares its token — `*_as`, "decided as" the row it is
//! handed — and declared in [`CARRIERS`], so its callers' names are read.
//! No door table: geom-core's doors are the kernel's own (`k_stats`),
//! and its names reach their callers' doors through the callers' crates.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source::{FORWARDS, NameCarrier, crate_dir, predicate_census, roster_violations};

/// The carriers that hand a name to a funnel from somewhere other than
/// the call's first argument.
const CARRIERS: &[NameCarrier] = &[
    NameCarrier::Const("SITE"),
    // The forwarders a `FORWARDS` line names, declared so their callers'
    // names are read (`roster_violations` holds each line to one).
    NameCarrier::Call("divisor"),
    NameCarrier::Call("forward"),
    NameCarrier::Call("registered"),
    NameCarrier::Call("render_as"),
    NameCarrier::Call("turned_between"),
    // The unit-direction row the normalising doors read at.
    NameCarrier::Const("UNIT_DIRECTION_ARM"),
    // The rigidity check's table of rows, read in a loop.
    NameCarrier::Const("COL0_UNIT"),
    NameCarrier::Const("COL1_UNIT"),
    NameCarrier::Const("COL2_UNIT"),
    NameCarrier::Const("COL01_ORTH"),
    NameCarrier::Const("COL12_ORTH"),
    NameCarrier::Const("COL02_ORTH"),
    NameCarrier::Const("DET_PLUS_ONE"),
    // Doors whose name is not their first argument.
    NameCarrier::Call("decide_unit_direction"),
    NameCarrier::Call("decide_direction_by"),
    // The straddle row a door mints, and a turn rule's two rows.
    NameCarrier::Call("straddle"),
    NameCarrier::Call("bounded"),
    // The identity ladder's unit-test reader, handed `(name, value)` rows.
    NameCarrier::Call("read_as"),
    // `Extent::forward_by_arc`'s name triple, its callers' rows.
    NameCarrier::Call("forward_by_arc"),
    NameCarrier::Const("ARC_ROWS"),
];

const ROW_FIELD: &str = "a field of a row struct, whose names are read at the struct's literal";
const IDENTITY: &str = "a `read_identity` call over data whose names are read where it is built";
const NOT_FIRST: &str = "a declared carrier's call whose name is not its first argument, named \
                         here by that first argument; its literal names are read";
const ROW_TABLE: &str = "a loop over the function's own table of rows, each a declared `const` \
                         carrier the census reads";
const PROVE_DOOR: &str = "`k_stats::prove`'s own name, handed to the door; the census reads no \
                          `prove` call's name yet \
                          (`work/onecopy/the-census-reads-no-prove-or-choose-call.md`)";
const CROSS_CRATE: &str = "a public forwarder whose callers are in other crates, whose rosters \
                           declare it a carrier and read their literals";
const LOOP_ROWS: &str = "a loop variable over the test's own literal rows";
const CARRIER_CALL: &str = "a declared carrier's call holding no literal name: the name comes from \
                            its enclosing function's caller or a binding, named here by the call's \
                            first argument";
const NOT_A_NAME: &str = "another `decide*` call, whose first argument is not a name";

const CLOSURE_FORWARDS: &str = "a closure bound in this `fn` hands on its callers' names, read where \
                                its callers call it (a declared carrier, or a `decide*` spelling); \
                                keyed by its `fn` until the census keys a closure by its binding \
                                (`work/onecopy/the-census-keys-no-closure-forwarder-by-its-binding.md`)";

/// The indirect sites, as `<file>: <expr>`, with what carries the name.
const INDIRECT: &[(&str, &str)] = &[
    (
        "ladder.rs: the_first_definite_datum_answers_and_all_zero_holds: &[]",
        NOT_A_NAME,
    ),
    ("k_stats.rs: decide_across: names", FORWARDS),
    ("k_stats.rs: decide_across_reported: far", FORWARDS),
    ("k_stats.rs: decide_across_reported: floor", FORWARDS),
    ("k_stats.rs: decide_magnitude: name", FORWARDS),
    ("k_stats.rs: decide_passing: name", FORWARDS),
    ("k_stats.rs: prove: name", PROVE_DOOR),
    (
        "ladder.rs: an_agreeing_ladder_reads_every_rung_and_checks_its_answers: name",
        LOOP_ROWS,
    ),
    ("ladder.rs: data_beneath_sum: straddle", CROSS_CRATE),
    ("ladder.rs: decide_magnitude: name", FORWARDS),
    ("ladder.rs: decided: name", FORWARDS),
    (
        "ladder.rs: read_identity#1(data.iter().map(|&(n, m)| (n, Margin::of(m))))",
        IDENTITY,
    ),
    (
        "ladder.rs: read_identity#2(data.iter().map(|&(n, m)| (n, Margin::of(Probe(m)))))",
        IDENTITY,
    ),
    ("ladder.rs: read_identity: name", FORWARDS),
    (
        "linalg/rigid_motion.rs: an_undecided_check_ends_in_the_lever_and_notes_a_poisoned_margin: 0.5 * (band().zero() + band().escalate())",
        CARRIER_CALL,
    ),
    (
        "linalg/rigid_motion.rs: an_undecided_check_ends_in_the_lever_and_notes_a_poisoned_margin: check",
        CLOSURE_FORWARDS,
    ),
    (
        "linalg/rigid_motion.rs: an_undecided_check_ends_in_the_lever_and_notes_a_poisoned_margin: f64::NAN",
        CARRIER_CALL,
    ),
    ("linalg/rigid_motion.rs: check_rigid: check", ROW_TABLE),
    ("linalg/rigid_motion.rs: turned_between: site", FORWARDS),
    ("linalg/rigid_motion.rs: turned_onto: self.w", CARRIER_CALL),
    ("linalg/unit_vec.rs: decide_direction_by: site", FORWARDS),
    ("linalg/unit_vec.rs: decide_unit_direction: v", NOT_FIRST),
    ("linalg/unit_vec.rs: levered: v", NOT_FIRST),
    ("linalg/unit_vec.rs: new: v", NOT_FIRST),
    ("linalg/unit_vec.rs: new_scaled: v * scale", NOT_FIRST),
    (
        "linalg/unit_vec.rs: the_direction_door_tells_an_underflowed_length_from_a_zero_one: good",
        NOT_FIRST,
    ),
    (
        "linalg/unit_vec.rs: the_direction_door_tells_an_underflowed_length_from_a_zero_one: v",
        NOT_FIRST,
    ),
    ("line2.rs: meet: det", NOT_A_NAME),
    ("period.rs: decide: name", FORWARDS),
    ("period.rs: decide_signed: name", FORWARDS),
    ("period.rs: forward: name", FORWARDS),
    ("period.rs: forward_by_arc: chord_name", FORWARDS),
    ("period.rs: forward_by_arc: name", FORWARDS),
    ("period.rs: forward_by_arc: sense_name", FORWARDS),
    ("period.rs: read: rows.straddle", ROW_FIELD),
    ("period.rs: read: rows.turn", ROW_FIELD),
    ("period.rs: registered: turn", FORWARDS),
    ("running.rs: divisor: name", FORWARDS),
    (
        "sym.rs: register_equal: return SymRegistration::Witnessed;",
        NOT_A_NAME,
    ),
    ("sym.rs: sign_within: band", NOT_A_NAME),
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
