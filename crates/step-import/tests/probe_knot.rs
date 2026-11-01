//! REVIEW PROBE (B8b), now a standing row: a REACHABLE hostile knot
//! multiplicity.
//!
//! `entities.rs::knot_vector` run-length-expands the file's stated
//! multiplicity. The review asked whether that is reachable and what
//! happens: it is (a `B_SPLINE_CURVE_WITH_KNOTS` is a subset curve
//! type, reached through an ordinary `EDGE_CURVE`), and it used to
//! ABORT — `(2000000000, 2000000000)` asks for a 16 GB allocation and
//! the allocator answers `abort`. That is strictly worse than a panic:
//! `catch_unwind` cannot see it, so the crate's headline no-panic row
//! could not have caught the class. This file was `#[ignore]`d
//! *because* it killed the test binary.
//!
//! The fix (M7-4 fix pass, finding MINOR-1) bounds the expansion by
//! the count ISO 10303-42 fixes — `control_points + degree + 1` for a
//! clamped B-spline — in checked arithmetic BEFORE anything is
//! allocated. So the probe flips: it is no longer ignored, and it
//! asserts the typed refusal rather than documenting the abort.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::{Duration, Instant};

use geom_core::Tol;
use step_import::{ImportOptions, StepImportError, import_step};

fn box_step() -> String {
    std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/box.step"
    ))
    .unwrap()
}

/// Runs one probe under `catch_unwind` with a time budget, answering
/// the import's own result. The panic and hang assertions stay — they
/// are what the probe was written to check — and the refusal
/// assertions sit on top of them rather than replacing them.
fn run(tag: &str, text: &str) -> Result<f64, StepImportError> {
    let t = Instant::now();
    let owned = text.to_owned();
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        import_step(&owned, &ImportOptions::default(), Tol::witness()).map(|i| i.eps_in())
    }));
    let took = t.elapsed();
    assert!(out.is_ok(), "{tag}: PANICKED in {took:?}");
    assert!(took < Duration::from_secs(30), "{tag}: HUNG ({took:?})");
    // The bound compares two integers read straight from the record,
    // so the refusal is immediate whatever the stated multiplicity. A
    // budget that "worked" by taking a minute would still be a
    // denial-of-service door, so the cost is pinned as not depending
    // on the number the file wrote.
    assert!(
        took < Duration::from_secs(5),
        "{tag}: took {took:?} — the multiplicity check must not depend on the \
         multiplicity's SIZE"
    );
    out.unwrap()
}

/// Replaces the box's edge #21 carrier with a B-spline curve carrying
/// the given multiplicity list — a curve type inside the subset. The
/// curve has TWO control points (`#23`, `#25`), so at degree 1 the
/// schema's knot count is 2 + 1 + 1 = 4.
fn with_bspline(mults: &str, knots: &str, degree: usize) -> String {
    box_step().replace(
        "#26 = LINE('',#27,#28);",
        &format!(
            "#26 = B_SPLINE_CURVE_WITH_KNOTS('',{degree},(#23,#25),\
             .UNSPECIFIED.,.F.,.F.,({mults}),({knots}),.UNSPECIFIED.);"
        ),
    )
}

/// Every multiplicity list that does not sum to the schema's count
/// refuses TYPED, naming the entity — from an off-by-one to the two
/// hostile ones that used to abort the process.
#[test]
fn hostile_knot_multiplicities_refuse_typed_instead_of_aborting() {
    // The well-formed control: multiplicities summing to exactly
    // 2 + 1 + 1 = 4. It must NOT be refused by the new bound — a
    // budget that rejects valid curves would be a regression dressed
    // as hardening. (It may still refuse further down the pipeline,
    // where a spline carrier meets a box's planar adoption; what this
    // row pins is that any refusal there is not the knot budget.)
    if let Err(e) = run("well-formed bspline", &with_bspline("2,2", "0.,1.", 1)) {
        assert!(
            !e.to_string().contains("knot multiplicities summing to"),
            "a schema-valid knot vector must pass the budget: {e}"
        );
    }

    // The two that used to reach a 16 GB allocation, the usize-overflow
    // shape, and an ordinary off-by-one. One rule covers all four,
    // which is the point: the bound is the schema's own count, not a
    // "sane cap" someone has to keep tuning.
    for (tag, mults) in [
        ("mult 2e9 each", "2000000000,2000000000"),
        ("mult 1e9", "1000000000,2"),
        ("mult 1e18", "1000000000000000000,2"),
        ("mult overflow", "18446744073709551615,18446744073709551615"),
        ("off by one", "3,2"),
    ] {
        let err = run(tag, &with_bspline(mults, "0.,1.", 1))
            .expect_err("a knot count the schema does not permit must refuse");
        match &err {
            StepImportError::MalformedRecord { id, expected } => {
                assert_eq!(*id, 26, "{tag}: the refusal names the curve entity");
                assert!(
                    expected.contains("control_points + degree + 1"),
                    "{tag}: and states the schema's count: {expected}"
                );
            }
            other => panic!("{tag}: expected MalformedRecord, got: {other}"),
        }
    }
}
