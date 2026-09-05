//! **No signature on the shell's offset chain names an `f64`
//! epsilon** — the D4 ¶1 witness rule, made mechanical for the one
//! chain that used to break it.
//!
//! The shell doors took a `tolerance: f64` beside `tol: Tol` and passed
//! it down through the face-replacement doors to the fit lane, so two
//! callers of one verb could fit against two different epsilons and
//! nothing said so. They do not any more: the witness travels and the
//! value is read once, at `props::fit_precision`, which is the last
//! kernel-side door before `geom-brep`'s fit engine.
//!
//! The compiler is the receipt for the doors that exist today — a
//! caller has no number to pass. What the compiler cannot say is that a
//! LATER edit did not add one back, which is what this suite is: the
//! chain, read as code, must contain no parameter whose type is `f64`
//! and whose name reads as a tolerance.
//!
//! **What it cannot see, stated.** An epsilon named nothing like one
//! (`slack: f64`, `budget: f64`); one smuggled inside a struct or a
//! tuple; one reached through a type alias for `f64`; a door moved OUT
//! of the sentinel region in `props.rs`; and the chain growing a file
//! the roster below does not name — that roster is hand-kept, exactly
//! like the census it guards, and a new module on the chain is a visit
//! here that nothing mechanical demands.
//!
//! And it stops at the kernel's edge, deliberately. `geom-brep`'s fit
//! engine below the lane takes its target as a number, because its own
//! suite measures the refinement, budget and limb ladder at chosen
//! ones; what this suite guards is that the KERNEL never chooses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

/// The chain, file by file: the verb doors, the face-replacement doors
/// they call, and the lane door that ends the kernel's half of it.
///
/// `shell.rs` and `replace_face.rs` are read WHOLE — every door in
/// either is on this chain. `props.rs` is not: it hosts the quadrature
/// lane too, whose ε reads are a different chain, so only the region
/// its sentinels bracket is read (the `NODE-TAG-SPACE` precedent). A
/// door moved out of that region walks past this suite, which is the
/// price of reading a region rather than a file and is what the
/// sentinel comment at the site says not to do.
const CHAIN: [(&str, &str); 2] = [
    ("shell.rs", include_str!("../src/shell.rs")),
    ("replace_face.rs", include_str!("../src/replace_face.rs")),
];

/// `props.rs`, whole — the sentinel region is cut out of it below.
const PROPS: &str = include_str!("../src/props.rs");

/// The lane's stretch of the chain: the two fit doors and the one read.
fn lane_region() -> &'static str {
    PROPS
        .split_once("SHELL-TOLERANCE-CHAIN BEGIN")
        .expect("the lane carries its opening sentinel")
        .1
        .split_once("SHELL-TOLERANCE-CHAIN END")
        .expect("the lane carries its closing sentinel")
        .0
}

/// A parameter name that reads as a tolerance. Deliberately broader
/// than the one spelling that was there, so a rename does not walk
/// past this row.
fn reads_as_a_tolerance(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    ["tolerance", "eps", "epsilon", "precision"]
        .iter()
        .any(|w| n.contains(w))
}

/// **No `f64` tolerance parameter anywhere on the chain.**
///
/// Comments and string literals are blanked through the shared Rust
/// reader before the scan, so the prose that explains why the parameter
/// is gone does not read as the parameter coming back.
#[test]
fn no_signature_on_the_shell_chain_names_an_f64_epsilon() {
    let mut hits: Vec<String> = Vec::new();
    let mut chain: Vec<(&str, &str)> = CHAIN.to_vec();
    chain.push(("props.rs (lane region)", lane_region()));
    for (file, source) in chain {
        let code = test_utils::source::code_and_literals(source);
        for (n, line) in code.lines().enumerate() {
            let Some((lhs, rhs)) = line.split_once(':') else {
                continue;
            };
            if !rhs.trim_start().starts_with("f64") {
                continue;
            }
            let name = lhs.trim().trim_start_matches('_');
            if reads_as_a_tolerance(name) {
                hits.push(format!("{file}:{}: {}", n + 1, line.trim()));
            }
        }
    }
    assert!(
        hits.is_empty(),
        "an f64 epsilon is back on the shell's offset chain, where the Tol witness is the only \
         tolerance a signature may take (D4 ¶1):\n{}",
        hits.join("\n")
    );
}

/// **The value is read once, and the site is named.**
///
/// `fit_precision` is where the witness becomes a number for the fit
/// engine below; a second read on the chain would mean two doors
/// deriving the target separately, which is the shape the witness
/// exists to prevent. The other `tol.eps()` reads in these files are
/// not on this chain — they are the quadrature lane's and the offset
/// doors' own decide margins — so this row counts the fit lane's calls
/// rather than every read in the file.
#[test]
fn the_fit_target_is_read_at_one_site() {
    let props = test_utils::source::code_and_literals(lane_region());
    let reads = props
        .lines()
        .filter(|l| l.contains("fn fit_precision"))
        .count();
    assert_eq!(
        reads, 1,
        "the fit target's one read site is gone or twinned; it is `props::fit_precision`"
    );
    let calls = props.matches("fit_precision(tol)").count();
    assert_eq!(
        calls, 2,
        "the lane has two fit doors — the mint and the re-derivation — and both must reach the \
         target through the one read; found {calls} call(s)"
    );
}
