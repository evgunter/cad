//! **R1 review probe (PR #264, C4), retired into its successor (M8
//! instancing).**
//!
//! What it pinned: dm1's refusal was exactly the M7-4 Leg D instancing
//! gate, fired at the second differing transform (#186, bolt_4), AFTER
//! the whole shape resolved — the executable form of PR #264's
//! "geometry distance retired" claim.
//!
//! That gate is gone. `REPRESENTATION_RELATIONSHIP` says which
//! component each `ITEM_DEFINED_TRANSFORMATION` places (`rep_1`), so
//! dm1's 7 occurrences of its 3 breps materialize as 7 placed
//! instances instead of refusing at the second map. This file keeps
//! the same subject and asserts the successor claims:
//!
//! 1. **the gate is gone, not moved** — no refusal of dm1 mentions
//!    assembly instancing, and none names #186;
//! 2. **the seven instances are read as seven** — the file's own
//!    occurrence structure, counted from the text, is what the resolver
//!    walks (3 component representations, 7 relationships, each naming
//!    one of them);
//! 3. **the placement layer is now BEHIND the geometry** — dm1's
//!    remaining refusal is reachable only once every instance's frame
//!    was read and applied. Since #327 (stage-1 CURVE recognition)
//!    that refusal has moved AGAIN, and past the whole D7 ladder: the
//!    file's rational-quadratic rim carriers are recognized as circles
//!    and promoted, every edge of every instance adopts, every pcurve
//!    mints and certifies, and the first thing that refuses is the
//!    SHARED AT-REST GATE — `VolumeUncomputable` /
//!    `QuadratureBudget` on the rational cylinder wall. That lane is
//!    no longer missing and the miss is no longer a floor — the
//!    enclosure quarters cleanly per refinement round — but the round
//!    budget is fixed, and this wall is still inside a factor of two
//!    of the ambient target when it runs out. This probe pins where
//!    the frontier actually is, so a claim that it moved has to be
//!    executable too.
//!
//! Per-instance placement CORRECTNESS (each frame on its own component
//! and no other) is pinned where a file that IMPORTS can carry it:
//! `freecad.rs::refusals_survive_the_dialect_relaxations` (d), on
//! planted mutations of `twobody_importexport`'s real transforms.
//!
//! **This row is dm1's only unconditional import in the suite.**
//! Importing dm1 costs ~30× the rest of the wild refusal corpus put
//! together, so the rows that used to re-import it for a WEAKER
//! statement now point here instead:
//! `wild::wild_refusals_are_typed_and_name_their_class` skips it (its
//! entity-naming check moved into the `TierInvalid` arm below), and
//! `review_probes_m7_3`'s V6 first-refusal-site probe is retired — it
//! asserted a strict subset of this row. `tier_gate.rs` still sweeps
//! the file at three ε_in values and pins BOTH ε cells' message
//! fragments there.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use std::path::PathBuf;
use step_import::{ImportOptions, StepImportError, import_step};

fn dm1() -> String {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "wild",
        "stepcode",
        "dm1-id-214.stp",
    ]
    .iter()
    .collect();
    std::fs::read_to_string(&path).unwrap()
}

#[test]
fn dm1_no_longer_refuses_at_the_instancing_gate() {
    let text = dm1();

    // (2) The file really does state seven occurrences of three
    // component representations — counted from the text, so the claim
    // does not rest on the reader agreeing with itself.
    let occurrences = text.matches("NEXT_ASSEMBLY_USAGE_OCCURRENCE").count();
    let transforms = text.matches("ITEM_DEFINED_TRANSFORMATION").count();
    let breps = text.matches("MANIFOLD_SOLID_BREP").count();
    assert_eq!(occurrences, 7, "l-bracket + 3 bolts + 3 nuts");
    assert_eq!(transforms, 7, "one per occurrence");
    assert_eq!(breps, 3, "three component representations, seven instances");

    // (1) and (3): the disposition — a two-cell claim, because the
    // ambient band selects which frontier is first (see the coarse
    // arm below).
    let coarse = geom_core::Tol::witness().get().eps > 1e-9;
    match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Err(StepImportError::Structure { id, what }) => {
            panic!("the assembly layer must not refuse dm1 any more: #{id} {what}")
        }
        // **The coarse-band cell** (#327's ε sweep, third outcome).
        // Retiring #685 let the ladder reach edges it had never
        // reached at any band, and at a COARSE ambient ε the first of
        // them — `#389`, a two-point `QUASI_UNIFORM_CURVE` polyline
        // that stays NURBS — is offered ZERO candidates. That is a
        // GAP, not a refusal, and it is PRE-EXISTING: nothing #327
        // touches can reach a degree-1 open carrier (the circle
        // estimator refuses an open curve before it estimates
        // anything), and the edge was simply masked behind #685 at
        // every band until now. Named, pinned, and filed rather than
        // averaged away — the cell says which outcome belongs to
        // which band, which is the whole point of sweeping.
        Err(StepImportError::Adoption { id, attempts }) => {
            assert!(
                coarse,
                "at a fine ambient band the D7 ladder must not refuse dm1 at all \
                 (#327 retired edge #685): #{id}, {} candidate(s)",
                attempts.len()
            );
            assert_eq!(id, 389, "the coarse-band cell's edge");
            assert!(
                attempts.is_empty(),
                "and it is the polyline GAP, not a refusal with candidates"
            );
        }
        Err(StepImportError::TierInvalid { solid, errors }) => {
            assert!(
                !coarse,
                "the coarse band's cell is the #389 ladder gap, not the gate: {errors:?}"
            );
            // Adopted from `wild::wild_refusals_are_typed_and_name_their_class`,
            // which no longer imports this file. INVARIANT: every
            // typed refusal points at something in the file a reader
            // can go and look at — the gate's verdict names the
            // `MANIFOLD_SOLID_BREP` it was asked about, and carries at
            // least one kernel verdict inside naming what it is about.
            assert!(
                solid.is_some_and(|id| id > 0) && !errors.is_empty(),
                "the refusal must name an entity: solid {solid:?}, verdicts {errors:?}"
            );
            let shown = StepImportError::TierInvalid { solid, errors }.to_string();
            assert!(
                shown.contains("the certified quadrature enclosure stalled at"),
                "the frontier is now the rational-patch-flux lane, not the ladder: {shown}"
            );
        }
        other => panic!("dm1's refusal has moved out of the at-rest gate; got {other:?}"),
    }
}
