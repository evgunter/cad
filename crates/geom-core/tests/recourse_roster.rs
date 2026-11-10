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
//! No door table: geom-core's doors are the kernel's own (`k_stats`),
//! and its names reach their callers' doors through the callers' crates.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source::{NameCarrier, crate_dir, predicate_census, roster_violations};

/// The carriers that hand a name to a funnel from somewhere other than
/// the call's first argument.
const CARRIERS: &[NameCarrier] = &[
    // The unit-direction row the normalising doors read at.
    NameCarrier::Const("UNIT_DIRECTION_ARM"),
    // Doors whose name is not their first argument.
    NameCarrier::Call("decide_unit_direction"),
    NameCarrier::Call("decide_direction_by"),
    // The straddle row a door mints, and a turn rule's two rows.
    NameCarrier::Call("straddle"),
    NameCarrier::Call("bounded"),
];

const FORWARDS: &str = "a funnel handing on the name its caller gives it, read where its callers \
                        call a declared carrier; one entry stands for every such site in the \
                        file, so a new forwarder under an undeclared token here is not seen";
const ROW_FIELD: &str = "a field of a row struct, whose names are read at the struct's literal";
const IDENTITY: &str = "a `read_identity` call over data whose names are read where it is built";
const NOT_FIRST: &str = "a declared carrier's call whose name is not its first argument, named \
                         here by that first argument; its literal names are read";
const NOT_A_NAME: &str = "another `decide*` call, whose first argument is not a name";

/// The indirect sites, as `<file>: <expr>`, with what carries the name.
const INDIRECT: &[(&str, &str)] = &[
    ("k_stats.rs: far", FORWARDS),
    ("k_stats.rs: floor", FORWARDS),
    ("k_stats.rs: name", FORWARDS),
    ("k_stats.rs: names", FORWARDS),
    ("ladder.rs: name", FORWARDS),
    ("ladder.rs: straddle", FORWARDS),
    (
        "ladder.rs: read_identity#1(data.iter().map(|&(n, m)| (n, Margin::of(m))))",
        IDENTITY,
    ),
    (
        "ladder.rs: read_identity#2(data.iter().map(|&(n, m)| (n, Margin::of(Probe(m)))))",
        IDENTITY,
    ),
    ("linalg/rigid_motion.rs: check", FORWARDS),
    ("linalg/rigid_motion.rs: site", FORWARDS),
    ("linalg/unit_vec.rs: good", NOT_FIRST),
    ("linalg/unit_vec.rs: site", FORWARDS),
    ("linalg/unit_vec.rs: v", NOT_FIRST),
    ("linalg/unit_vec.rs: v * scale", NOT_FIRST),
    ("line2.rs: det", NOT_A_NAME),
    ("period.rs: name", FORWARDS),
    ("period.rs: rows.straddle", ROW_FIELD),
    ("period.rs: rows.turn", ROW_FIELD),
    ("running.rs: name", FORWARDS),
    ("sym.rs: band", NOT_A_NAME),
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
