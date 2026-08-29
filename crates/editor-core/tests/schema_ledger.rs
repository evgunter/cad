//! The schema ledger's guard: for every version the kernel claims, an
//! entry, a golden, and a number that agree.
//!
//! **Why this exists.** The version is ONE LINE of constant, so two
//! units claiming the same number merge CLEAN — git compares bytes, not
//! meanings. The per-version paragraphs on `SCHEMA_VERSION` are where
//! the two claims can be told apart, which also makes deleting one a
//! way to resolve the conflict; that happened once and no check saw it.
//! Nothing else in the tree asserts the ledger is complete.
//!
//! These rows live here, beside the other per-version suites, rather
//! than as unit tests in `persist/mod.rs` — that file is scheduled for
//! comment trimming (S38 / L2 of `docs/SMELL-SCAN-2026-08.md`), and a
//! guard housed inside its own subject reads as more of the prose being
//! trimmed.
//!
//! **What no row here checks:** whether `SCHEMA_VERSION` holds the
//! RIGHT number. Two units can still claim the same one — their entries
//! collide in prose, but nothing compares this branch against main.
//! That is a by-eye read at the final re-merge, and nothing automates
//! it.

use editor_core::SCHEMA_VERSION;

/// The ledger's PROSE — the entries are doc comments, so the module is
/// read through [`test_utils::source::comments_only`], the view that
/// blanks code and literals and leaves the docs.
///
/// The view is what makes the search mean what it says: an entry is a
/// paragraph a reader will find, so a `format!("Version {n} is …")` in
/// code, or the heading quoted inside a string, must not answer for
/// one. [`test_utils::source::code_only`] would blank exactly what
/// this row looks for, which is why the class needs the inverse view
/// and not one helper.
///
/// `include_str!` of one known file, where `topo::sector_shape`'s
/// twin-guard deliberately walks `src/` instead: that guard's subject
/// can legitimately re-fork into a third file, and this one cannot. The
/// ledger has exactly one home, and moving it should fail this row
/// rather than silently narrow it. The failure mode of the wrong guess
/// here is a false RED, which is the safe direction.
fn ledger() -> String {
    test_utils::source::comments_only(include_str!("../src/persist/mod.rs"))
}

/// The committed save for version `n`, if there is one.
fn golden(n: u32) -> Option<String> {
    let path = format!(
        "{}/tests/golden/v{n}_golden.cad",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(path).ok()
}

/// **Every version from 2 up carries a ledger entry.**
///
/// v1 is excluded deliberately: it is the origin format, described in
/// the module docs rather than as a bump from anything.
///
/// What this cannot catch: it reads for a heading, not for meaning. An
/// entry that describes the wrong format, contradicts the break it
/// names, or is a bare placeholder line passes.
#[test]
fn every_version_has_a_ledger_entry() {
    let prose = ledger();
    let missing: Vec<u32> = (2..=SCHEMA_VERSION)
        .filter(|n| !prose.contains(&format!("Version {n} is")))
        .collect();
    assert!(
        missing.is_empty(),
        "schema ledger has no entry for version(s) {missing:?} — every bump owes one. \
         An entry lost to a conflict resolution is recoverable from the branch that \
         claimed that number."
    );
}

/// **Every version the ledger claims has a golden that says so.**
///
/// This is the half a string search cannot fake: the golden is a real
/// save, and its header is the version the format actually shipped
/// under. A bump whose golden was never written, or was written under
/// the previous number, fails here.
#[test]
fn every_version_has_a_golden_carrying_its_own_number() {
    let mut bad: Vec<String> = Vec::new();
    for n in 1..=SCHEMA_VERSION {
        match golden(n) {
            None => bad.push(format!("v{n}: no golden")),
            Some(text) => {
                let header = text.lines().next().unwrap_or_default();
                if header != format!("schema: {n}") {
                    bad.push(format!("v{n}: golden header is {header:?}"));
                }
            }
        }
    }
    assert!(
        bad.is_empty(),
        "goldens disagree with the versions the kernel claims: {bad:?}"
    );
}

/// **No golden above the current version.**
///
/// The direction the two rows above cannot see: a bump rolled back, or
/// a golden regenerated from a branch that claimed a number main never
/// took, leaves a file for a version this build does not support.
#[test]
fn no_golden_claims_a_version_above_the_constant() {
    let stray: Vec<u32> = (SCHEMA_VERSION + 1..SCHEMA_VERSION + 8)
        .filter(|n| golden(*n).is_some())
        .collect();
    assert!(
        stray.is_empty(),
        "golden(s) exist for version(s) {stray:?}, above SCHEMA_VERSION {SCHEMA_VERSION} — \
         either the constant was rolled back or a golden was written under a number main \
         never took"
    );
}
