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
//! No door table: geom's names reach their callers' doors through the
//! callers' own crates.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source::{NameCarrier, crate_dir, predicate_census, roster_violations};

/// The funnels whose literal arguments are names.
const CARRIERS: &[NameCarrier] = &[
    // A spiric regime's and a ring torus's rows, named at their readers'
    // `sites`.
    NameCarrier::Call("rows_as"),
    NameCarrier::Call("stable_azimuth_as"),
    NameCarrier::Call("off_as"),
    NameCarrier::Call("tube_half_as"),
    NameCarrier::Call("read_as"),
];

const FORWARDS: &str = "a funnel handing on the name its caller gives it, read where its callers \
                        call a declared carrier; one entry stands for every such site in the \
                        file, so a new forwarder under an undeclared token here is not seen";
const ROW_FIELD: &str = "a field of a row struct, whose names are read at the struct's literal";
const CALLERS_CRATE: &str = "a public door whose caller names the row (`Ray::ahead`), read in the \
                             calling crate's own census";

/// The indirect sites, as `<file>: <expr>`, with what carries the name.
const INDIRECT: &[(&str, &str)] = &[
    ("azimuth.rs: row", FORWARDS),
    ("chart_point.rs: name", FORWARDS),
    ("curves.rs: site", CALLERS_CRATE),
    ("curves.rs: sites.off_axis", ROW_FIELD),
    ("curves.rs: sites.two_ovals", ROW_FIELD),
    ("surfaces.rs: RingSites::CONVENTION.tube", ROW_FIELD),
    ("surfaces.rs: name", FORWARDS),
    ("surfaces.rs: sites.tube", ROW_FIELD),
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
